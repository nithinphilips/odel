//! # Odel
//!
//! Odel is a tool to upload Data Integrator files to IBM TRIRIGA.
//!

#![recursion_limit = "1024"]

mod errors;
mod soap;
mod tririga;
mod utils;


#[allow(unused_imports)]
#[macro_use] extern crate log;
#[macro_use] extern crate clap;
#[macro_use] extern crate anyhow;

use anyhow::{Result, Context};
use backoff::ExponentialBackoff;
use backoff::backoff::Backoff;
use clap::{AppSettings, App};
use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;
use tririga::ResponseHelperExt;
use tririga::transport::HttpTransport;
use tririga::tririga::RunNamedQuery;
use std::borrow::Borrow;
use crate::tririga::TririgaEnvironment;

const FILE_NAME_MAX_LEN: usize = 50;
const TRIRIGA_AUTH_OBJECTID: &str = "1000";
#[allow(dead_code)]
const TRIRIGA_AUTH_LOGIN_ACTIONID: &str = "1005";
const TRIRIGA_AUTH_FORCELOGIN_ACTIONID: &str = "1006";

const DI_SUCCESS_STATUS_EN_US: &str = "Rollup All Completed";
const MAX_RETRIES: &str = "23";

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}

fn build_cli() -> App<'static, 'static> {
    clap_app!(odel =>
        (name: "odel")
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())

        (@arg verbose: -v --verbose ...
        "Print information verbosely. Define multiple times to get more verbose. \
                    You may define -v up to 3 times.")
        (@arg quiet: -q --quiet conflicts_with("verbose") "Disable all informational messages")

        (@arg shell_completion: +takes_value --("shell-completion") hidden(true)
            conflicts_with ("DATAFILE") possible_values(&clap::Shell::variants())
            case_insensitive(true)
            "Print shell completion script")

        (@arg username: +takes_value -u --("username") "TRIRIGA Username. Default: system")
        (@arg password: +takes_value -p --("password") "TRIRIGA Password. Default: admin")
        (@arg url: +takes_value -l --("url") "TRIRIGA URL. Default: http://localhost:9080")
        (@arg config: +takes_value -c --("config") "Path to server configuration file. Default: \
        tririga.json. See CONFIG FILE")

        (@arg max_retries: +takes_value -r --("max-retries") "The maximum number of times to check \
        for file processing completion. Default 23. See WAIT below.")

        (@arg no_wait: -w --("no-wait") "Do not Wait until the file is processed by TRIRIGA. \
        See WAIT below")

        (@arg module: +takes_value -m --("module") "The name of the record's module you are \
        uploading")
        (@arg businessobject: +takes_value -b --("businessobject") "The name of the record's \
        business object you are uploading")
        (@arg form: +takes_value -f --("form") "The name of the record's form you are uploading")
        (@arg action: +takes_value -a --("action") "The action to apply to the newly created \
        records. See ACTION below.")

        (@arg DATAFILE: "Sets the data file to upload. Only Tab-delimited files are supported. If \
        empty, data will be read from STDIN")

        (after_help:
r#"DATAFILE:
    The DATAFILE must be in tab-delimited format. Odel does not verify the file format. If the
    the file format is not correct the uploaded file may fail to import in TRIRIGA.

SUPPORTED PLATFORM:
    Any recent TRIRIGA platform version is supported. We have tested with 3.6.1.
    The TRIRIGA instance must have a Connector for Business Applications license.

FILENAME:
    The module, businessobject and form options are optional if the file is named in one of
    these patterns

    * <module>-<businessObject>-<form>.txt
    * <prefix>-<module>-<businessObject>-<form>.txt
    * <module>-<businessObject>.txt

    Otherwise, you must specify them using the --module, --businessobject and --form options.
    If the file name pattern is recognized, it takes precedence over the command-line options.

ACTION:
    The --action option is optional. If omitted the available actions are retrieved from TRIRIGA
    and the first action is applied. This may not be predictable.

    If you specify an invalid action, Odel will warn you, but will upload anyways. This could
    lead to the creation of invalid records in the TRIRIGA.

WAIT:
    By default Odel will wait until Data Integrator has completed processing the file. This means
    that the records you uploaded are created in the system. This does NOT mean that any
    asynchronous processing workflows that the record creation may trigger have finished.

    This capability can be used to upload related records in a batch (such as Building and child
    Spaces.) You can be sure that subsequent files are not uploaded until the preceding records
    are created in the system. The EXIT CODE set by Odel can be used to stop the processing if any
    file in the chain fails.

    You can disable waiting by setting the --no-wait flag.

    The upload processing status check is done by polling the status of matching Data Upload
    records. This is done in exponentially slowing interval. This prevents Odel from making too
    many requests to TRIRIGA when uploading large files while also quickly getting the status
    for small files. This can be configured using the --max-retries option. The default value is
    23 (requests). In terms of time, --max-retires=23 will cause Odel to wait about 24 hours before
    giving up.

    If the retry time has elapsed and the file still has not been processed, Odel will exit
    with an error.

CONFIG FILE:
    Odel can read read the url, username and password values from a JSON file. By default
    Odel will search for a file named `tririga.json` in CWD and if found will load it. You can load
    another file by using the --config option.

    The file format should be:

        {
            "webHost" : "http://localhost:9080/",
            "webUsername" : "system",
            "webPassword" : "admin"
        }

    If you set the --url, --username or --password command-line options, they will always override
    the values read from the JSON file.

SSO:
    SAML SSO Authentication is not supported. You must have direct access to the TRIRIGA
    server. LDAP authentication with username and password may work, but is untested.

EXIT CODES:
    0 if the upload was successful and the file was processed.
    1 if the upload failed or if the maximum wait time had elapsed.

    If the --no-wait flag is specified, the exit code 0 only indicates that the file
    has been sent to TRIRIGA. It may still fail to process in TRIRIGA."#)
    (long_version: concat!(
        "v", crate_version!(), " ", env!("VERGEN_TARGET_TRIPLE"), "\n",
        "Copyright (C) 2020 ", crate_authors!(), "\n",
        "License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.", "\n",
        "This is free software: you are free to change and redistribute it.", "\n",
        "There is NO WARRANTY, to the extent permitted by law.", "\n\n",
        "Built on ", env!("VERGEN_BUILD_DATE"), " from ", env!("VERGEN_SHA"), "\n",
        "https://github.com/nithinphilips/odel"
     ))
    )
    .setting(AppSettings::ColoredHelp)
    .setting(AppSettings::ColorAlways)
}

async fn run() -> Result<()> {
    let matches = build_cli().get_matches();

    if matches.is_present("shell_completion") {
        let shell = value_t!(matches, "shell_completion", clap::Shell)
            .expect("Shell completion name");
        build_cli().gen_completions_to(
            build_cli().get_bin_name().unwrap_or_else(|| clap::crate_name!()),
            shell,
            &mut std::io::stdout());
        std::process::exit(0);
    };

    // Adding 1 to set the default level to WARN. Default is 0 = ERROR
    let verbose = matches.occurrences_of("verbose") as usize + 1;
    let quiet = matches.is_present("quiet");

    let no_wait = matches.is_present("no_wait");
    let max_retries = matches.value_of("max_retries").unwrap_or_else(|| MAX_RETRIES);

    let max_retries = max_retries.parse::<usize>()
        .context("--max-retries value must be a number")?;


    stderrlog::new()
        .module(module_path!())
        .quiet(quiet)
        .verbosity(verbose)
        // .timestamp(stderrlog::Timestamp::Second)
        .init()?;

    let tririga_json_file_name = matches.value_of("config")
        .unwrap_or_else(|| tririga::TRIRIGA_JSON_FILENAME);

    let env = if Path::new(tririga_json_file_name).exists() {
        let mut tririga_json_f = File::open(tririga_json_file_name)?;
        let env: serde_json::Result<TririgaEnvironment> = serde_json::from_reader(tririga_json_f);
        match env {
            Ok(env) => {
                info!("{} file found in current directory. it will be used for host information \
                if CLI options are missing.", tririga_json_file_name);
                // trace!("{:?}", env); // Could leak credentials to log
                env
            }
            Err(e) => {
                warn!("{} file was found but could not be parsed. {}",
                      tririga_json_file_name, e);
                TririgaEnvironment::default()
            }
        }
    } else {
        info!("{} file not found in current directory.", tririga_json_file_name);
        TririgaEnvironment::default()
    };

    let url = match matches.value_of("url") {
        Some(s) => String::from(normalize_url(s)),
        None => {
            let t_url = env.web_host.ok_or(anyhow!("No webURL value set in {}",
            tririga::TRIRIGA_JSON_FILENAME))?;
            String::from(normalize_url(t_url.as_str()))
        }
    };

    let username = match matches.value_of("username") {
        Some(s) => String::from(s),
        None => env.web_username.ok_or(anyhow!("No webUsername value set in {}",
        tririga::TRIRIGA_JSON_FILENAME))?
    };

    let password = match matches.value_of("password") {
        Some(s) => String::from(s),
        None => env.web_password.ok_or(anyhow!("No webPassword value set in {}",
        tririga::TRIRIGA_JSON_FILENAME))?
    };

    info!("Connecting to {} as {}", url, username);

    let data_file = matches.value_of("DATAFILE").ok_or_else(|| anyhow!("Datafile is required"))?;

    if !Path::new(data_file).exists() {
        bail!("The data file '{}' does not exist.", data_file);
    }

    let web_t = HttpTransport{
        client: reqwest::Client::builder().cookie_store(true).build()?,
        url: url.to_string(),
        endpoint: format!("{}/ws/TririgaWS?wsdl", url),
        username: username.to_string(),
        password: password.to_string()
    };

    let soap_t = HttpTransport{
        client: reqwest::Client::builder().build()?,
        url: url.to_string(),
        endpoint: format!("{}/ws/TririgaWS?wsdl", url),
        username: username.to_string(),
        password: password.to_string()
    };

    let fc =
        if let Some(fcc) = parse_filename(data_file) {
            fcc
        } else {
            FileComponents {
                module: matches
                    .value_of("module")
                    .ok_or_else (|| anyhow!("The object type information could not be extracted \
                    from file name `{}`. Specify it using --module, --businessobject and  \
                    (optionally) --form.",
                    data_file))?
                    .to_string(),
                business_object: matches
                    .value_of("businessobject")
                    .ok_or_else(|| anyhow!("The object type information could not be extracted \
                    from file name `{}`. Specify it using --module, --businessobject and \
                    (optionally) --form.", data_file))?
                    .to_string(),
                form: match matches.value_of("form") {
                    Some(s) => Some(s.to_string()),
                    None => None
                },
            }
        };

    let action = matches.value_of("action");

    info!("Resolving object ids for {}::{}::{:?}", fc.module, fc.business_object, fc.form);
    let oi = get_object_info(&soap_t, &fc).await?;
    info!("Resolved object ids for {}({})::{}({})::{:?}({})",
          fc.module, oi.module_id, fc.business_object,
          oi.business_object_id, fc.form, oi.gui_id);

    let data_file_trimmed = upload_file(
        &web_t,
        data_file,
        action,
       &oi,
    ).await?;

    if !no_wait {
        get_upload_status(
            &soap_t,
            &data_file_trimmed,
            max_retries
        ).await
    }else {
        info!("Not waiting for processing. The file may be still being processed by TRIRIGA.");
        Ok(())
    }
}

async fn upload_file(
    tranport: &HttpTransport,
    data_file: &str,
    action: Option<&str>,
    objectinfo: &ObjectInfo,
) -> Result<String> {

    let login_params = vec![
        ("USERNAME", tranport.username.as_str()),
        ("PASSWORD", tranport.password.as_str()),
        ("objectId", TRIRIGA_AUTH_OBJECTID),
        ("actionId", TRIRIGA_AUTH_FORCELOGIN_ACTIONID)
    ];

    info!("DI: Create session");
    tranport.client.get(&format!("{}/login", &tranport.url)).send().await?;

    info!("DI: Login");
    tranport.client.post(&format!("{}/Authenticate.srv", &tranport.url))
        .form(&login_params)
        .send().await?;

    info!("DI: Get Security Token");
    let res = tranport.client.get(&format!("{}/html/en/default/common/dataUploadFile.jsp", &tranport
        .url))
        .send().await?;

    let security_name: String;
    let security_value: String;

    if let Ok(data) = res.text().await {
        let re =
            Regex::new(r#"new TririgaSecurity\("(?P<name>[^"]*)","(?P<value>[^"]*)"\);"#)?;
        let caps = re.captures(&data).ok_or_else(|| anyhow!("Error compiling Regex"))?;

        security_name = String::from(&caps["name"]);
        security_value = String::from(&caps["value"]);
    } else {
        // Maybe it's not necessary?
        security_name = String::from("");
        security_value = String::from("");
        warn!("Unable to extract Tririga security token. The Upload may fail.");
    }

    debug!("DI: Will POST using tokens: {}={}", security_name, security_value);

    let path = PathBuf::from(data_file);
    let file_name_only = path
        .file_name().ok_or_else(|| anyhow!("Error reading path"))?
        .to_str().ok_or_else(|| anyhow!("Error reading path"))?;

    let data_file_trimmed = trim_filename(file_name_only, FILE_NAME_MAX_LEN).
        ok_or_else(|| anyhow!("Unable to trim file name to TRIRIGA limits"))?;

    info!("DI: File name trimmed to '{}'", data_file_trimmed);

    let (module_id_str, business_object_id_str, gui_id_str) =
        (
            objectinfo.module_id.to_string(),
            objectinfo.business_object_id.to_string(),
            objectinfo.gui_id.to_string()
        );

    let action_name = match action {
        Some(a) => {
            if objectinfo.actions.iter().all(|x| x != a ){
                warn!("The given action '{}' is not valid. Valid values are {:?}. I will try to \
                upload anyways. TRIRIGA may create corrupt records in the database",
                a, objectinfo.actions.join(", "))
            }
            a
        },
        None => {
            let x = objectinfo.actions
                .first().ok_or_else(|| anyhow!("Unable to get default action name for object"))?;
            info!("Selected the first available action: {}", x);
            x
        }
    };

    info!("Using action: {}", action_name);

    let upload_params = vec![
        ("updateAct", "createSpec"),
        ("filenames", &data_file_trimmed),
        ("classTypeN", &module_id_str),
        ("objectTypeN", &business_object_id_str),
        ("guiIdN", &gui_id_str),
        ("delimiterN", ".TAB"),
        ("charSet", "UTF-8"),
        ("transactionTypeN", "Insert/New"),
        ("batch", "NO"),
        ("actionName", action_name),
        ("stateName", "triDraft")
    ];

    info!("DI: Reading file: '{}'", data_file);
    let mut f = File::open(data_file)?;
    let mut data = Vec::new();
    f.read_to_end(&mut data)?;

    let the_file = reqwest::multipart::Part::
        stream(data)
        .file_name(String::from(&data_file_trimmed))
        .mime_str("text/plain")?;

    let file_parts = reqwest::multipart::Form::new()
        .text("updateAct", "createSpec")
        .part("theFile", the_file);

    info!("DI: Upload file");
    tranport.client
        .post(&format!("{}/html/en/default/common/dataUploadFile.jsp?updateAct=createSpec", &tranport.url))
        .multipart(file_parts)
        .header(&security_name, &security_value)
        .send().await?;

    info!("DI: Create DI job");
    tranport.client.post(&format!("{}/html/en/default/common/dataSmartUpload.jsp", &tranport.url))
        .header(&security_name, &security_value)
        .form(&upload_params)
        .send().await?;

    Ok(data_file_trimmed)
}

#[derive(Debug)]
struct ObjectInfo {
    module_id: i64,
    business_object_id: i64,
    gui_id: i64,
    actions: Vec<String>
}

async fn get_object_info(t: &HttpTransport, fileinfo: &FileComponents) -> Result<ObjectInfo>
{
    use tririga::tririga::*;

    let params = GetModuleId{
        module_name: Some(fileinfo.module.to_string())
    };
    let module_id: i64 = get_module_id(t, &params)
        .await.context("Failed resolve module id")?.out as i64;

    let params = GetObjectTypeId {
        module_name: Some(fileinfo.module.to_string()),
        object_type_name: Some(fileinfo.business_object.to_string()),
    };
    let object_type_id = get_object_type_id(t, &params)
        .await.context("Failed to resolve business object id")?.out;

    let gui_id =
        match &fileinfo.form {
            Some(form) => {
                let params = GetGUIs {
                    object_type_id,
                };
                let guis = get_guis(t, &params)
                    .await.context("Failed to resolve form id")?;

                guis.out.gui
                .into_iter()
                .find(|g| g.name.as_ref() == Some(form))
                .ok_or_else(|| anyhow!("Error while getting form ID for {}", form))?
                .id.ok_or_else(|| anyhow!("Error while getting form ID for {}", form))?
            },
            None => {
                let params = GetDefaultGuiId {
                    object_type_id,
                };
                let guis = get_default_gui_id(t, &params)
                    .await.context("Failed to get default form id")?;
                debug!("Form name is not provided. Using the default form for the business object.");
                guis.out
            }
        };

    let params =  GetObjectTypeActions {
        module_id,
        object_type_id,
        record_id: -1,
        gui_id: -1
    };

    let actions = get_object_type_actions(t, &params)
        .await.context("Error getting form actions")?;

    let actions: Vec<String> =
        actions.out.ok_or_else(|| anyhow!("Error getting form actions"))?
        .bo_action_steps.ok_or_else(|| anyhow!("Error getting form actions"))?
        .object_type_action_step.into_iter()
        .filter(|x| x.action.is_some())
        .map(|x| x.action.unwrap())
        .collect();

    Ok(ObjectInfo {
        module_id,
        business_object_id: object_type_id,
        gui_id,
        actions
    })
}

async fn get_upload_status(t: &HttpTransport, filename: &str, max_retries: usize) -> Result<()>
{
    use tririga::tririga::*;
    use tririga::dto::*;

    let processing_statuses = vec!["NEW", "DONE", "UPLOADING..."];

    let params = RunNamedQuery{
        project_name: None,
        module_name: Some("System".to_string()),
        object_type_name: Some("Data Upload".to_string()),
        query_name: Some("Data Upload - System - Manager Query".to_string()),
        filters: ArrayOfFilter {
            filter: vec![
                Filter {
                    data_type: 320,
                    field_name: "File Name".to_string(),
                    operator: 10,
                    section_name: "General".to_string(),
                    value: filename.to_string()
                }
            ],
        },
        start: 0,
        maximum_result_count: 999,
    };

    let result = run_named_query(t, &params).await?;
    let result = result.out.ok_or_else(|| anyhow!("error getting upload status"))?
        .query_response_helpers.ok_or_else(|| anyhow!("error getting upload status"))?
        .query_response_helper;

    let most_recent_record = result
        .iter()
        .max_by(|a, b| a.record_id.unwrap().cmp(&b.record_id.unwrap()))
        .ok_or_else(|| anyhow!("unable to get the latest data upload record"))?;

    info!("Record {} appears to be the one we just uploaded", most_recent_record.record_id.unwrap());

    let mut last_status = most_recent_record.get_field_value("Status").unwrap().to_string();

    if processing_statuses
        .iter()
        .any(|&i| i == last_status)
    {
        info!("[0] Upload is not yet ready. Status is '{}'", last_status);

        let mut backoff = ExponentialBackoff::default();
        let mut iteration_ct = 0 as usize;

        loop {
            iteration_ct += 1;

            if iteration_ct > max_retries {
                return Err(errors::OdelError::Timeout(last_status).into())
            }

            //task::sleep(Duration::from_secs(1)).await;
            sleep(backoff.next_backoff().unwrap_or_else(|| Duration::from_secs(5)));

            info!("[{}] Upload is not yet ready. Status is '{}'", iteration_ct, last_status);

            last_status = get_upload_status_inner(
                t,
                &params,
                most_recent_record.record_id.unwrap()
            ).await?;

            if processing_statuses.iter() .all(|&i| i != last_status)
            {
                info!("Upload is ready. Status is '{}'", last_status);
                break;
            }
        }
    }

    if last_status == DI_SUCCESS_STATUS_EN_US {
        Ok(())
    } else {
        warn!("Upload failed");
        let link = format!("{}/pc/notify/link?recordId={}", t.url, most_recent_record.record_id.unwrap());
        Err(errors::OdelError::UploadFailed(last_status, link).into())
    }
}


async fn get_upload_status_inner(
    t: &HttpTransport,
    params: &RunNamedQuery,
    recent_record_id: i64
)
    -> Result<String> {
    use tririga::tririga::*;

    let result = run_named_query(t, params)
        .await?;
    let result = result.out.ok_or_else(|| anyhow!("error getting upload status"))?
        .query_response_helpers.ok_or_else(|| anyhow!("error getting upload status"))?
        .query_response_helper;

    let record = result
        .iter()
        .find(|x| x.record_id.unwrap() == recent_record_id)
        .ok_or_else(|| anyhow!("error getting upload status"))?;

    Ok(String::from(record.get_field_value("Status").unwrap()))
}

// --
// const FILE_NAME_MAX_LEN = 50;

/// Trim a file name to a maximum length while trying to preserve the
/// extension. The length is treated as character-wise, not byte-wise.
///
/// If Unicode multi-byte characters are given, the output will have
/// the correct number of characters (except in case of graphemes which
/// are treated as separate characters,) however the number of actual
/// bytes may vary.
///
/// In the case of TRIRIGA this method works when the database is
/// configured with CHAR semantics and may fail when it is using
/// BYTE semantics.
///
/// If the extension alone is longer than the maxlength, the extension
/// may be shortened or removed altogether.
///
/// Since we may be trimming in the middle, this function will always
/// return an owned string.
fn trim_filename(filename: &str, maxlength: usize) -> Option<String>
{
    if filename.len() <= maxlength {
        return Some(String::from(filename))
    }

    let path = Path::new(filename);
    if let Some(extension) = path.extension(){
        let extension = extension.to_str()?;
        let extension_len = extension.chars().count();
        if extension_len < maxlength {
            // If we have an ext, we probably have
            let file_stem = path.file_stem()?.to_str()?;
            let truncate_len = maxlength - extension_len - 1;
            let mut trunc_file_stem = String::with_capacity(maxlength);
            let tf1: String = file_stem.chars().take(truncate_len).collect();
            trunc_file_stem.push_str(&tf1);
            trunc_file_stem.push('.');
            trunc_file_stem.push_str(extension);
            return Some(trunc_file_stem)
        }
    }

    let result: String = filename.chars().take(maxlength).collect();
    Some(result)
}

#[derive(Debug)]
struct FileComponents {
    module: String,
    business_object: String,
    form: Option<String>,
}

///  Extract Module, BO, and Form name from filename.
///
///  The filename should be in the format ``Module-BusinessObject-Form.txt``
fn parse_filename(filename: &str) -> Option<FileComponents> {

    if filename.to_lowercase().contains("patchhelper"){
        let fc = FileComponents  {
            form: Some(String::from("triPatchHelper")),
            business_object: String::from("triPatchHelper"),
            module: String::from("triHelper"),
        };
        return Some(fc)
    }

    let path = PathBuf::from(filename);
    let file_stem = path.file_stem()?.to_str()?;
    let mut parts: Vec<&str> = file_stem.split('-').collect();

    parts.reverse();

    if parts.len() >= 3 {
        return Some(FileComponents  {
            form: Some(parts[0].trim().replace("_", " ")),
            business_object: parts[1].trim().replace("_", " "),
            module: parts[2].trim().replace("_", " "),
        })
    } else if parts.len() == 2 {
        return Some(FileComponents  {
            form: None,
            business_object: parts[0].trim().replace("_", " "),
            module: parts[1].trim().replace("_", " "),
        })
    }

    None
}

/// Normalizes a URL by stripping the trailing slash.
fn normalize_url(url: &str) -> &str
{
    if !url.is_empty() && url.ends_with('/') {
        let index = url.len() - 1;
        &url[..index]
    } else {
        url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_filename() {
        // Normal call
        assert_eq!(trim_filename("There-are-fifty-four-characters-in-this-file-name.txt", 10), Some(String::from("There-.txt")));

        // Another normal operation
        assert_eq!(trim_filename("There-are.txt", 10), Some(String::from("There-.txt")));

        // No trimming is done if the file length is less than or equal to the
        // max length:
        assert_eq!(trim_filename("There-.txt", 10), Some(String::from("There-.txt")));
        assert_eq!(trim_filename("There.txt", 10), Some(String::from("There.txt")));

        // If the extension part is very long it may be removed or trimmed:
        assert_eq!(trim_filename("a.a-very-long-file-ext", 10), Some(String::from("a.a-very-l")));
        assert_eq!(trim_filename("a-very-long-filename.a-very-long-file-ext", 10), Some(String::from("a-very-lon")));

        // Extension only file names are simply trimmed
        assert_eq!(trim_filename(".a-very-long-file-ext", 10), Some(String::from(".a-very-lo")));
        assert_eq!(trim_filename(".a-very-long-file-ext", 10000), Some(String::from(".a-very-long-file-ext")));

        // Edge cases
        assert_eq!(trim_filename("", 10000), Some(String::from("")));
        // Handles unicode multi-byte characters gracefully
        assert_eq!(trim_filename("♥♥♥♥.♥", 3), Some(String::from("♥.♥")));
    }

    #[test]
    fn test_parse_filename() {
        // File name only, no extension:
        if let Some(comps) = parse_filename("Module-BusinessObject-Form") {
            assert_eq!(comps.module, "Module");
            assert_eq!(comps.business_object, "BusinessObject");
            assert_eq!(comps.form, Some("Form".to_string()));
        } else{
            panic!("Parse failed!");
        }

        // File with spaces around parts, which are trimmed:
        if let Some(comps) = parse_filename(" Module - BusinessObject - Form ") {
            assert_eq!(comps.module, "Module");
            assert_eq!(comps.business_object, "BusinessObject");
            assert_eq!(comps.form, Some("Form".to_string()));
        } else{
            panic!("Parse failed!");
        }

        // Prefixes are allowed
        if let Some(comps) = parse_filename("001 - Module - BusinessObject - Form ") {
            assert_eq!(comps.module, "Module");
            assert_eq!(comps.business_object, "BusinessObject");
            assert_eq!(comps.form, Some("Form".to_string()));
        } else{
            panic!("Parse failed!");
        }

        // File name with prefix and extension
        if let Some(comps) = parse_filename("001 - Module - BusinessObject - Form.txt") {
            assert_eq!(comps.module, "Module");
            assert_eq!(comps.business_object, "BusinessObject");
            assert_eq!(comps.form, Some("Form".to_string()));
        } else{
            panic!("Parse failed!");
        }

        // File name with prefix and extension and unix path
        if let Some(comps) = parse_filename("/home/odel/001 - Module - BusinessObject - Form.txt") {
            assert_eq!(comps.module, "Module");
            assert_eq!(comps.business_object, "BusinessObject");
            assert_eq!(comps.form, Some("Form".to_string()));
        } else{
            panic!("Parse failed!");
        }

        // File name with prefix and extension and windows path
        if let Some(comps) = parse_filename("C:\\Users\\Odel\\001 - Module - BusinessObject - \
        Form.txt") {
            assert_eq!(comps.module, "Module");
            assert_eq!(comps.business_object, "BusinessObject");
            assert_eq!(comps.form, Some("Form".to_string()));
        } else{
            panic!("Parse failed!");
        }

        //   This method handles a special case. If the file does not have a three
        //   part identifier, but contains the word PatchHelper, it will return the
        //   info for the TRIRIGA Patch Helper module:
        if let Some(comps) = parse_filename("PatchHelper_10_5.txt") {
            assert_eq!(comps.module, "triHelper");
            assert_eq!(comps.business_object, "triPatchHelper");
            assert_eq!(comps.form, Some("triPatchHelper".to_string()));
        } else{
            panic!("Parse failed!");
        }

        //   It is not case sensitive or position dependent:
        if let Some(comps) = parse_filename("/home/odel/UpgradeApp_patchhelper.txt") {
            assert_eq!(comps.module, "triHelper");
            assert_eq!(comps.business_object, "triPatchHelper");
            assert_eq!(comps.form, Some("triPatchHelper".to_string()));
        } else{
            panic!("Parse failed!");
        }

        // Underscores in the file name will be interpreted as spaces. This will be
        // useful when calling Odel from Makefiles
        if let Some(comps) = parse_filename("Data_Utilities-triEventDTO-triEventDTO") {
            assert_eq!(comps.module, "Data Utilities");
            assert_eq!(comps.business_object, "triEventDTO");
            assert_eq!(comps.form, Some("triEventDTO".to_string()));
        } else{
            panic!("Parse failed!");
        }

        // If the filename only has two parts, form is omitted
        if let Some(comps) = parse_filename("Data_Utilities-triEventDTO.txt") {
            assert_eq!(comps.module, "Data Utilities");
            assert_eq!(comps.business_object, "triEventDTO");
            assert_eq!(comps.form, None);
        } else{
            panic!("Parse failed!");
        }

        // Invalid files names cause a None result
        assert!(parse_filename("Data_Utilities").is_none());
        assert!(parse_filename("").is_none());
    }

    #[test]
    fn test_normalize_url() {
        assert_eq!("http://www.google.com", normalize_url("http://www.google.com/"));
        assert_eq!("http://www.google.com", normalize_url("http://www.google.com"));
        assert_eq!("", normalize_url(""));
    }
}
