<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AddUser</name>
   <tag></tag>
   <elementGuidId>7cfc449c-b25b-45b8-8ac2-7719479628e9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;UserID\&quot;: \&quot;string\&quot;,\n  \&quot;Username\&quot;: \&quot;string\&quot;,\n  \&quot;FullName\&quot;: \&quot;string\&quot;,\n  \&quot;Email\&quot;: \&quot;user@example.com\&quot;,\n  \&quot;Password\&quot;: \&quot;string\&quot;,\n  \&quot;PhoneNumber\&quot;: \&quot;string\&quot;,\n  \&quot;IsMobileLoginAllowed\&quot;: true,\n  \&quot;IsWebLoginAllowed\&quot;: true,\n  \&quot;DataAccessGroupId\&quot;: 0,\n  \&quot;RoleId\&quot;: 0,\n  \&quot;RoleName\&quot;: \&quot;string\&quot;,\n  \&quot;IsEnabled\&quot;: true,\n  \&quot;UserRole\&quot;: \&quot;string\&quot;,\n  \&quot;UserRoleID\&quot;: \&quot;string\&quot;,\n  \&quot;UserUniqueID\&quot;: \&quot;string\&quot;,\n  \&quot;DataAccessGroupName\&quot;: \&quot;string\&quot;,\n  \&quot;EmailConfirmed\&quot;: \&quot;string\&quot;,\n  \&quot;BackgroundTheme\&quot;: \&quot;string\&quot;,\n  \&quot;EulaAcceptanceFlag\&quot;: true,\n  \&quot;PlaySoundFlag\&quot;: \&quot;string\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>23fc5645-4881-4830-bbb7-9c4574859708</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.8</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>​${GlobalVariable.baseUrl}:6001/​api​/User​/Add</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
