<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body</name>
   <tag></tag>
   <elementGuidId>a3d44a45-cc35-4335-abda-e8b893a51789</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>This website uses cookies to ensure you get the best experience on our website. Learn moreGot it!

    jQuery File Upload
    Download
    Source Code
    Documentation
    © blueimp.net

jQuery File Upload Demo

    Adobe Creative Cloud for Teams starting at $29.99 per month.ads via Carbon

jQuery UI version

    Theme:
    
        Black Tie
        Blitzer
        Cupertino
        Dark Hive
        Dot Luv
        Eggplant
        Excite Bike
        Flick
        Hot sneaks
        Humanity
        Le Frog
        Mint Choc
        Overcast
        Pepper Grinder
        Redmond
        Smoothness
        South Street
        Start
        Sunny
        Swanky Purse
        Trontastic
        UI Darkness
        UI Lightness
        Vader
    


    Basic
    Basic Plus
    Basic Plus UI
    AngularJS
    jQuery UI


    File Upload widget with multiple file selection, drag&amp;drop support, progress bars, validation and preview images, audio and video for jQuery UI.
    Supports cross-domain, chunked and resumable file uploads and client-side image resizing.
    Works with any server-side platform (PHP, Python, Ruby on Rails, Java, Node.js, Go etc.) that supports standard HTML form file uploads.



    
    &lt;input type=&quot;hidden&quot; name=&quot;redirect&quot; value=&quot;https://blueimp.github.io/jQuery-File-Upload/&quot;>
    
    
        
            
             
                Add files...
                
            
             Start upload
             Cancel upload
             Delete
            
            
            
        
        
        
            
            
            
             
        
    
    
    
        
            
        
        
            
                abc.jpg
            
            
        
        
            27.20 KB
            
        
        
            
                 Start
            
            
                 Cancel
            
        
    
Upload server currently unavailable - Tue Apr 23 2019 13:11:44 GMT+0700 (SE Asia Standard Time)

Demo Notes

    The maximum file size for uploads in this demo is 999 KB (default file size is unlimited).
    Only image files (JPG, GIF, PNG) are allowed in this demo (by default there is no file type restriction).
    Uploaded files will be deleted automatically after 5 minutes or less (demo files are stored in memory).
    You can drag &amp; drop files from your desktop on this webpage (see Browser support).
    Please refer to the project website and documentation for more information.
    Built with jQuery UI.



    
    
    ‹
    ›
    ×
    
    



{% for (var i=0, file; file=o.files[i]; i++) { %}
    &lt;tr class=&quot;template-upload fade&quot;>
        &lt;td>
            &lt;span class=&quot;preview&quot;>&lt;/span>
        &lt;/td>
        &lt;td>
            {% if (window.orientation === undefined || !o.options.loadImageFileTypes.test(file.type)) { %}
                &lt;p class=&quot;name&quot;>{%=file.name%}&lt;/p>
            {% } %}
            &lt;strong class=&quot;error&quot;>&lt;/strong>
        &lt;/td>
        &lt;td>
            &lt;p class=&quot;size&quot;>Processing...&lt;/p>
            &lt;div class=&quot;progress&quot;>&lt;/div>
        &lt;/td>
        &lt;td>
            {% if (!i &amp;&amp; !o.options.autoUpload) { %}
                &lt;button class=&quot;start&quot; disabled>Start&lt;/button>
            {% } %}
            {% if (!i) { %}
                &lt;button class=&quot;cancel&quot;>Cancel&lt;/button>
            {% } %}
        &lt;/td>
    &lt;/tr>
{% } %}



{% for (var i=0, file; file=o.files[i]; i++) { %}
    &lt;tr class=&quot;template-download fade&quot;>
        &lt;td>
            &lt;span class=&quot;preview&quot;>
                {% if (file.thumbnailUrl) { %}
                    &lt;a href=&quot;{%=file.url%}&quot; title=&quot;{%=file.name%}&quot; download=&quot;{%=file.name%}&quot; data-gallery>&lt;img src=&quot;{%=file.thumbnailUrl%}&quot;>&lt;/a>
                {% } %}
            &lt;/span>
        &lt;/td>
        &lt;td>
            {% if (window.orientation === undefined || !file.thumbnailUrl) { %}
                &lt;p class=&quot;name&quot;>
                    &lt;a href=&quot;{%=file.url%}&quot; title=&quot;{%=file.name%}&quot; download=&quot;{%=file.name%}&quot; {%=file.thumbnailUrl?'data-gallery':''%}>{%=file.name%}&lt;/a>
                &lt;/p>
            {% } %}
            {% if (file.error) { %}
                &lt;div>&lt;span class=&quot;error&quot;>Error&lt;/span> {%=file.error%}&lt;/div>
            {% } %}
        &lt;/td>
        &lt;td>
            &lt;span class=&quot;size&quot;>{%=o.formatFileSize(file.size)%}&lt;/span>
        &lt;/td>
        &lt;td>
            &lt;button class=&quot;delete&quot; data-type=&quot;{%=file.deleteType%}&quot; data-url=&quot;{%=file.deleteUrl%}&quot;{% if (file.deleteWithCredentials) { %} data-xhr-fields='{&quot;withCredentials&quot;:true}'{% } %}>Delete&lt;/button>
            &lt;input type=&quot;checkbox&quot; name=&quot;delete&quot; value=&quot;1&quot; class=&quot;toggle&quot;>
        &lt;/td>
    &lt;/tr>
{% } %}
































// Initialize the jQuery UI theme switcher:
$('#theme-switcher').change(function () {
    var theme = $('#theme');
    theme.prop(
        'href',
        theme.prop('href').replace(
            /[\w\-]+\/jquery-ui.css/,
            $(this).val() + '/jquery-ui.css'
        )
    );
});




window.addEventListener('load',function(){window.cookieconsent.initialise({palette:{popup:{background:'#fff'},button:{background:'#222'}}})})


/html[1]/body[1]</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
   </webElementXpaths>
</WebElementEntity>
