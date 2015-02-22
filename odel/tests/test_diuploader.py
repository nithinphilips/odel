import unittest

from odel import diuploader

class TestTrimFilename(unittest.TestCase):
    def test_trim_filename(self):
        expected = 'There-.txt'
        actual = diuploader.trim_filename("There-are-fifty-four-characters-in-this-file-name.txt",
            maxlength=10)
        self.assertEqual(expected, actual)

        actual = diuploader.trim_filename("There-are.txt",
            maxlength=10)
        self.assertEqual(expected, actual)

    def test_trim_filename_no_trim(self):
        expected = 'There-.txt'
        actual = diuploader.trim_filename("There-.txt",
            maxlength=10)
        self.assertEqual(expected, actual)

        expected = 'There.txt'
        actual = diuploader.trim_filename("There.txt",
            maxlength=10)
        self.assertEqual(expected, actual)

    def test_trim_filename_long_ext(self):
        expected = 'a.a-very-l'
        actual = diuploader.trim_filename("a.a-very-long-file-ext",
            maxlength=10)
        self.assertEqual(expected, actual)

        expected = 'a-very-lon'
        actual = diuploader.trim_filename("a-very-long-filename.a-very-long-file-ext",
            maxlength=10)
        self.assertEqual(expected, actual)

class TestParseFilename(unittest.TestCase):
    def test_parse_filename(self):
        # self.assertEqual(expected, parse_filename(filename, separator))
        assert True # TODO: implement your test here

class TestNormalizeUrl(unittest.TestCase):
    def test_normalize_url(self):
        # self.assertEqual(expected, normalize_url(url))
        assert True # TODO: implement your test here

class TestParseUrl(unittest.TestCase):
    def test_parse_url(self):
        # self.assertEqual(expected, parse_url(url, port))
        assert True # TODO: implement your test here

class TestUpload(unittest.TestCase):
    def test_upload(self):
        # self.assertEqual(expected, upload(url, filename, username, password, module, businessobject, form, action, no_wait))
        assert True # TODO: implement your test here

class TestMultipartMimeFilter(unittest.TestCase):
    def test_received(self):
        # multipart_mime_filter = MultipartMimeFilter()
        # self.assertEqual(expected, multipart_mime_filter.received(context))
        assert True # TODO: implement your test here

class TestGetObjectInfo(unittest.TestCase):
    def test_get_object_info(self):
        # self.assertEqual(expected, get_object_info(module, businessobject, form, site_url, username, password))
        assert True # TODO: implement your test here

class TestWaitForUpload(unittest.TestCase):
    def test_wait_for_upload(self):
        # self.assertEqual(expected, wait_for_upload(filename, site_url, username, password))
        assert True # TODO: implement your test here

class TestQueryResponseToDict(unittest.TestCase):
    def test_query_response_to_dict(self):
        # self.assertEqual(expected, query_response_to_dict(results))
        assert True # TODO: implement your test here

if __name__ == '__main__':
    unittest.main()
