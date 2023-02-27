import unittest
from selenium import webdriver
from selenium.webdriver.common.keys import Keys


class TestGoogleSearch(unittest.TestCase):

    def setUp(self):
        self.driver = webdriver.Chrome()
        self.driver.get("https://www.google.com/")

    def test_search(self):
        search_box = self.driver.find_element_by_name("q")
        search_box.send_keys("selenium")
        search_box.send_keys(Keys.RETURN)
        assert "selenium" in self.driver.title

    def tearDown(self):
        self.driver.quit()


if __name__ == '__main__':
    unittest.main()
