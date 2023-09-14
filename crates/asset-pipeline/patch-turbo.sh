# Hotwired Turbo doesn't pass CSP. Patch it.
if ! grep -F '/**this.installStylesheetElement();**/' node_modules/@hotwired/turbo/dist/*.js
then
    sed -i 's/this.installStylesheetElement();/\/**this.installStylesheetElement();**\//g' node_modules/@hotwired/turbo/dist/*.js 
    sed -i 's/this.installProgressElement();/\/**this.installProgressElement();**\//g' node_modules/@hotwired/turbo/dist/*.js 
fi