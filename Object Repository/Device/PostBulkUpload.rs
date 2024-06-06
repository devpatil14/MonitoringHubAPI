<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PostBulkUpload</name>
   <tag></tag>
   <elementGuidId>23b14274-b50c-4385-a4ba-fdbf52c957e7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;userID\&quot;: \&quot;string\&quot;,\n  \&quot;userUniqueId\&quot;: \&quot;string\&quot;,\n  \&quot;createdOn\&quot;: \&quot;string\&quot;,\n  \&quot;data\&quot;: [\n    {\n      \&quot;brandName\&quot;: \&quot;string\&quot;,\n      \&quot;privateIPAddress\&quot;: \&quot;string\&quot;,\n      \&quot;communicationPort\&quot;: \&quot;string\&quot;,\n      \&quot;userName\&quot;: \&quot;string\&quot;,\n      \&quot;password\&quot;: \&quot;string\&quot;,\n      \&quot;port\&quot;: \&quot;string\&quot;,\n      \&quot;channelNumber\&quot;: \&quot;string\&quot;,\n      \&quot;cameraName\&quot;: \&quot;string\&quot;,\n      \&quot;siteName\&quot;: \&quot;string\&quot;,\n      \&quot;isTranscoded\&quot;: \&quot;string\&quot;,\n      \&quot;rtspURL\&quot;: \&quot;string\&quot;,\n      \&quot;interfaceName\&quot;: \&quot;string\&quot;,\n      \&quot;message\&quot;: \&quot;string\&quot;,\n      \&quot;status\&quot;: \&quot;string\&quot;,\n      \&quot;strName\&quot;: \&quot;string\&quot;,\n      \&quot;cctvtype\&quot;: \&quot;string\&quot;,\n      \&quot;httpPort\&quot;: \&quot;string\&quot;,\n      \&quot;isTwoWayEnabled\&quot;: \&quot;string\&quot;,\n      \&quot;playbackChannelNumber\&quot;: \&quot;string\&quot;,\n      \&quot;isPlaybackTranscoded\&quot;: \&quot;string\&quot;,\n      \&quot;liveStreamTypeNumber\&quot;: \&quot;string\&quot;,\n      \&quot;playbackStreamTypeNumber\&quot;: \&quot;string\&quot;\n    }\n  ]\n}&quot;,
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
      <webElementGuid>befa9a75-3a50-4bae-a63a-44b585ba88c8</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.8</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}:6001/api/Device/DeviceBulkUpload</restUrl>
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
