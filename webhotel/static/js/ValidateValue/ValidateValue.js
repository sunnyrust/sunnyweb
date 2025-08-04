function ValidateValue(textbox) {
    var IllegalString = "[`~!#$^&*()=|{}':;',\\[\\].<>/?~！#￥……&*（）——|{}【】‘；：”“'。，、？]‘'";
    var textboxvalue = textbox.value;
    var index = textboxvalue.length - 1;

    var s = textbox.value.charAt(index);

    if (IllegalString.indexOf(s) >= 0) {
        s = textboxvalue.substring(0, index);
        textbox.value = s;
    }
}