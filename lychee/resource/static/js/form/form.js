$('.form-body').height($('.form-box').height() - $('.form-head').height() - 20);
$(window).on('resize',function(){
    $('.form-body').height($('.form-box').height() - $('.form-head').height() - 20);
});