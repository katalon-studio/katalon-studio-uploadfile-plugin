import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

String path = RunConfiguration.getProjectDir() + "/Data Files/"
path = path.replace("/", "\\")

//-----------------------------verify drag and drop on dropzonejs (works on firefox/chrome)----------------------------------------
WebUI.openBrowser('https://www.dropzonejs.com/')
CustomKeywords.'com.kms.katalon.keyword.uploadfile.UploadFile.uploadFileUsingRobot'(findTestObject('Page_Dropzonejs/input'), 
    path + "image1.jpg")

//-----------------------------verify upload file on jQuery-File-Upload----------------------------------------
WebUI.navigateToUrl('http://blueimp.github.io/jQuery-File-Upload/')
CustomKeywords.'com.kms.katalon.keyword.uploadfile.UploadFile.uploadFile'(findTestObject('Page_jQuery File Upload/input'), path + "image1.jpg")								
									
//using uploadFileUsingRobot function, works on firefox only
CustomKeywords.'com.kms.katalon.keyword.uploadfile.UploadFile.uploadFileUsingRobot'(findTestObject('Page_jQuery File Upload/input'), 
   path + "image2.jpg")

//-----------------------------verify upload file on flowjs----------------------------------------
WebUI.navigateToUrl('http://flowjs.github.io/ng-flow/')
CustomKeywords.'com.kms.katalon.keyword.uploadfile.UploadFile.uploadFile'(findTestObject('Page_flowjs/input_Upload_File'), path + "image1.jpg")						
//using uploadFileUsingRobot function, works on firefox only
CustomKeywords.'com.kms.katalon.keyword.uploadfile.UploadFile.uploadFileUsingRobot'(findTestObject('Page_flowjs/input_Upload_File'), 
    path + "image2.jpg")

//-----------------------------verify upload file on plupload----------------------------------------
WebUI.navigateToUrl('https://www.plupload.com/examples')
CustomKeywords.'com.kms.katalon.keyword.uploadfile.UploadFile.uploadFile'(findTestObject('Object Repository/Page_Plupload/input'), path + "image1.jpg")						
//using uploadFileUsingRobot function, works on firefox only
CustomKeywords.'com.kms.katalon.keyword.uploadfile.UploadFile.uploadFileUsingRobot'(findTestObject('Object Repository/Page_Plupload/input'),
	path + "image2.jpg")

//-----------------------------verify upload file on html5----------------------------------------
WebUI.navigateToUrl('http://demo.guru99.com/test/upload/')
//WebUI.uploadFile(findTestObject('Page_File Upload Demo/input'), path + "image1.jpg")
CustomKeywords.'com.kms.katalon.keyword.uploadfile.UploadFile.uploadFile'(findTestObject('Page_File Upload Demo/input'), path + "image1.jpg")					
//using uploadFileUsingRobot function, works on firefox only
CustomKeywords.'com.kms.katalon.keyword.uploadfile.UploadFile.uploadFileUsingRobot'(findTestObject('Page_File Upload Demo/input'),
   path + "image2.jpg")
