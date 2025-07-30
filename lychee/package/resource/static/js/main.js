var Ai_ServerJs=(function(){
	function mainJs(){
		this.ajaxurl = "";
	};

	/*首页js*/
	mainJs.prototype.index=function(){
		this.iframeHeight();
		this.treeStyle();
		this.treeInit();
		this.out();
		this.dropDown();
		this.showActive();
	};

	// 手机验证
	mainJs.prototype.reg=function(){
		this.regNumber()
	}
	/*首页iframe自适应高度*/
	mainJs.prototype.iframeHeight=function(){
		function autoHeight(){
			var ifm = document.getElementById("mainFrame");
        	var subWeb = document.frames ? document.frames["mainFrame"].document : ifm.contentDocument;

        	$(ifm).height(0);

	        if (ifm != null && subWeb != null) {
	        	$(ifm).height(subWeb.body.scrollHeight);
	        }
		}

		$(window).on('resize',function(){
			autoHeight();
		});

        $('#mainFrame').load(function(){
        	autoHeight();
        });
	};


	/*目录tree初始化*/
	mainJs.prototype.treeInit=function(){
		var simpleTreeCollection;
        simpleTreeCollection = $('.simpleTree').simpleTree({
            autoclose : true,
            afterClick : function(node) {
                var href = $('span:first', node).attr("href");
                if (href) {
                    $("#mainFrame").attr("src", href).parent();
                } else {
                    $(".simpleTree").get(0).nodeToggle(node.get(0));
                }
            },
            animate : false,
            drag : false
        });
	}


	/*目录鼠标移动样式*/
	mainJs.prototype.treeStyle=function(){
		//目录树子元素鼠标移动，父元素不变
        $('.leftbar_ul > li > span').hover(function(){
        	if(!$(this).parent().hasClass("folder-open")){
				$(this).addClass('mainNav_hover');
        	}
        },function(){
        	$(this).removeClass('mainNav_hover');
        })

        $('.leftbar_sonul > li > span').click(function(){
        	$(this).parent().addClass('leftbar_click').siblings().removeClass('leftbar_click').end().parent().parent().siblings().find('li').removeClass('leftbar_click');
		});
	}


	/*out index*/
	mainJs.prototype.out=function(){
		$('#indexOut').click(function(){
			return confirm('确定要退出吗？');
		});
	}


	/*员工头像上传*/
	mainJs.prototype.uploadHead=function(msg){
		$('#avatar').on('change', function() {
			if($(this).val().length>0){
				//校验图片格式
				var imgtype=/jpg|jpeg|gif|png/i;
				var suffix = $(this).val().split('.').splice(-1)[0];

				var fileId = "avatar";
				var dom = document.getElementById(fileId);
				var fileSize = dom.files[0].size;
				var imgsize = fileSize/(1024*1024)

				if (imgsize > 3){
					$(this).val("");
					alert("上传照片不能大于3M！");
					return ;
				}

				if(!imgtype.test(suffix)){
					$(this).val("");
					alert("请上传正确类型的照片格式！");
					return ;
				}


				var file = this.files[0];

			     //创建读取文件的对象  
			    var reader = new FileReader();  
			  
			    //创建文件读取相关的变量  
			    var imgFile;  
			  
			    //为文件读取成功设置事件  
			    reader.onload=function(e) {  	    
			        imgFile = e.target.result;  
			        $("#sucess_img").attr('src', imgFile);  
	
			        $('.upload_imgBox').css({'background':'#fff'});			
			    };  
						
			  
			  	//正式读取文件  
					reader.readAsDataURL(file);   
			}else{		
				if($('#form_editImg').length>0){
					$("#sucess_img").attr('src', $('#form_editImg').attr('src'));
				}else{
					$("#sucess_img").attr('src', ' ');
					$('.upload_imgBox').css({'background':'url(/static/Images/form_1.png) no-repeat center'})
				}
			}
		});


	    //编辑状态下加载图片
	    var imgSrc=$('#form_editImg').attr('src');

		$("#sucess_img").attr('src', imgSrc);


		var imgSrc2=$('#sucess_img').attr('src');
		if(imgSrc2!=' '){
			$('.upload_imgBox').css({'background':'#fff'});
		}
	}


	/*员工头像公共上传*/
	mainJs.prototype.uploadHeadPub=function(obj){
		var edimg = obj.parents('.upload_imgBox').next();

		obj.on('change', function() {
			if($(this).val().length>0){
				//校验图片格式
				var imgtype=/jpg|jpeg|gif|png/i;
				var suffix = $(this).val().split('.').splice(-1)[0];

				// var fileId = "avatar";
				// var dom = document.getElementById(fileId);
				var fileSize = obj.get(0).files[0].size;
				var imgsize = fileSize/(1024*1024)

				if (imgsize > 3){
					$(this).val("");
					alert("上传照片不能大于3M！");
					return ;
				}

				if(!imgtype.test(suffix)){
					$(this).val("");
					alert("请上传正确类型的照片格式！");
					return ;
				}


				var file = this.files[0];

			     //创建读取文件的对象
			    var reader = new FileReader();

			    //创建文件读取相关的变量
			    var imgFile;

			    //为文件读取成功设置事件
			    reader.onload=function(e) {
			        imgFile = e.target.result;
			        obj.prev().attr('src', imgFile);

			        obj.parents('.upload_imgBox').css({'background':'#fff'});
			    };


			  	//正式读取文件
		   	 	reader.readAsDataURL(file);
			}else{
				if(edimg.length>0){
					obj.prev().attr('src', edimg.attr('src'));
				}else{
					obj.prev().attr('src', ' ');
					obj.parents('.upload_imgBox').css({'background':'url(/static/Images/form_1.png) no-repeat center'})
				}
			}
		});


	    //编辑状态下加载图片
	    var imgSrc=edimg.attr('src');

		obj.prev().attr('src', imgSrc);


		var imgSrc2=obj.prev().attr('src');
		if(imgSrc2!=' '){
			obj.parents('.upload_imgBox').css({'background':'#fff'});
		}
	}
	
	/**
	 * 分页处理，包括跳转，搜索跳转----适用于get请求
	 * total-总页数 cur-当前页 tips-提示信息 hrefn-搜索之后的分页函数
	 */
	mainJs.prototype.getpage = function(total,cur,tips){

		var _this = this;
		total = parseInt(total);

		if(total==0 || isNaN(total)){
			total=1;
			cur=1;
		}

        /*使用url.min.js.插件,只需要切换page即可*/
        function pages(page){
        	var url = URI(document.location.href).removeQuery("p").addQuery("p", page);
                      window.location.href = url.toString();
        }

		$("#page").bootstrapPaginator({
	      	currentPage: cur,
	      	totalPages: total,
	      	bootstrapMajorVersion: 3,
	      	size: "small",
	      	onPageClicked: function(e,originalEvent,type,page){
	      		if(page != cur){
	      			 pages(page);
	      		}
	      	}
	    });

	    //跳转
	    $('#page_sBtn').click(function(){
	    	var num = parseInt($('#page_skip').val());
            var total = $('#page_num').text();

            if(isNaN(num) || num == ''){
                alert(tips[0]);
                $('#page_skip').val('');
                return ;
            }else if(num > total || num <= 0){
            	alert(tips[1]);
                $('#page_skip').val('');
                return ;
            }else if(num == cur){
            	alert(tips[2]);
            	$('#page_skip').val('');
            	return ;
            }

            $("#page").bootstrapPaginator("show",num);

            pages(num);
	    });

	    //支持回车
	    $('#page_skip').keydown(function(event) {
            if(event.keyCode==13){
                $('#page_sBtn').trigger('click');
            }
        });

        //如果只有一页就隐藏分页
        if(total == 1){
        	$('.tab_footer').css("visibility","hidden");
        }else{
        	$('.tab_footer').css("visibility","visible");
        }
	}

    // mark:limei
    // mainJs.prototype.dropDown = function(){
    // 	// dropdown mouseover
    // 	$(".header_info").on('mouseover',function(){
    // 		$(this).children('#login_drop').fadeIn().stop(true,true);
    // 	}).on('mouseleave',function(){
    // 		$(this).children('#login_drop').fadeOut().stop(true,true);
    // 	});

    // 	// 添加图标
    // 	var $lists = $('.leftbar_ul > li');
    // 	for(var i=0;i<$lists.length;i++){
    // 		$lists.eq(i).children('span').after('<em class="iconfont icon2">&#xe600;</em>');
    // 		$lists.eq(i).children('span').before('<em class="iconfont icon1"></em>');
    // 		var $data = $lists.eq(i).children('span').data('icon');
    // 		$('.icon1').eq(i).addClass($data);
    // 	};

	//      //箭头图标切换
	//      var click = true;
	//      var $text = $('.leftbar_ul>li>span')
	//      for(var i=0;i<$text.length;i++){
	//      	$text.eq(i).on('click',function(){
	//      		if(click||$(this).parent().hasClass('folder-open')==true||$(this).parent().hasClass('folder-open-last')==true){
	//      			$(this).next('.icon2').html('&#xe601;').parent().siblings().children('.icon2').html('&#xe600;');
	//      			click = false;
	//      		}else if(click==false||$(this).parent().hasClass('folder-close')==true||$(this).parent().hasClass('folder-close-last')==true){
	//      			$(this).next('.icon2').html('&#xe600;');
	//      			click = true;
	//      		}
	//      	})
	//      }

    // 	//当前menu被选中得时候，颜色为提示色蓝色
	// 	var $menu = $('.text').siblings('ul').children('li').children('span')
	// 	for(var i=0;i<$menu.length;i++){
	// 		$menu.eq(i).before('<em class="iconfont font_pos">&#xe664;</em>');
	// 	}
    // 	$menu.on('click',function(){
    // 		if($(this).hasClass('active')==true){
	// 				$(this).css({'color':'#1890ff'}).parent().siblings().children('span').css({"color":"#666"});
	// 				// $(this).addClass('activeColor').parent().siblings().children('span').addClass('unactiveColor');
	// 				// $(this).parent().addClass('activeBg').siblings().addClass('unactiveBg');
	// 				$(this).prev('.font_pos').html('&#xe7c2;').css({'color':'#1890ff'}).parent().siblings().children('em').html('&#xe664;').css({"color":"#666"});
    // 				$(this).parent().css({'background':'#e6f7ff','borderRight':'2px solid #1890ff'}).siblings().css({'background':'#fff','borderRight':'none'});
    // 				$(this).parent().parent().parent().siblings().children('ul').children('li').css({'background':'#fff','borderRight':'none'});
	// 				$(this).parent().parent().parent().siblings().children('ul').children('li').children('span').css({'color':'#666'});
	// 				$(this).prev('.font_pos').html('&#xe7c2;').parent().parent().parent().siblings().children('ul').children('li').children('em').html('&#xe664;').css({'color':'#666'});
    // 		}
    // 	})

    // }

	/* 门禁大屏幕背景上传
	 * 支持图片和视频
	 */
	mainJs.prototype.screen=function(){
		 //1.正则匹配图片或者视频
		 var imgtype=/jpg|jpeg|gif|png/;
		 var videotype=/mp4|avi|mpg|asf|mpeg|rmvb|mkv|mov/;

		 var type=$('#screen_welcome').attr('data-type').substring(1);
		 //初始化时欢迎语内容
		 var welcome_content=$('#screen_welcome').val();
		 var isfileUp=false; //文件是否上传

		 notUpload();

		 //2.未上传
		 function notUpload(){
		 	// $('#uploadImg_btn').addClass('notClickBtn').attr('disabled','disabled');

			isfileUp=false;

			if(imgtype.test(type)){
				$('#sucess_img').attr({"src":$('#form_editImg').attr('src')});
				$('.upload_imgBox').show();
			   	$('#video_box').hide();
				//$('#screen_wp').show();
			}else if(videotype.test(type)){
				$('.upload_imgBox').hide();
			   	$('#video_box').show();
				//$('#screen_wp').hide().val("");
			}
		 }



		 //3.已上传
		 function upload(suffix,file){
		 	//打开按钮
		 	// $('#uploadImg_btn').removeClass('notClickBtn').removeAttr('disabled');

		 	//创建读取文件的对象
		    var reader = new FileReader();

		 	//判断文件类型img or video
		 	if(imgtype.test(suffix)){
		 		//通知服务器的文件类型
		 		$('#filetype').val('img');

		 		//为文件读取成功设置事件
			    reader.onload=function(e) {
			        var imgFile = e.target.result;
			        $("#sucess_img").attr('src', imgFile);
			    };

			    $('.upload_imgBox').show();
			   	$('#video_box').hide();
			    //$('#screen_wp').show();
				$('#uploadImg_btn').removeClass('notClickBtn').removeAttr('disabled');
	 			//正式读取文件
	   	 		reader.readAsDataURL(file);

	   	 		// if($.trim($('#screen_welcome').val())==''){
	   	 		// 	$('#uploadImg_btn').addClass('notClickBtn').attr('disabled','disabled');
	   	 		// }

		 	}else if(videotype.test(suffix)){
		 		$('#filetype').val('video');

		 		$('.upload_imgBox').hide();
			   	$('#video_box').show();
				//$('#screen_wp').hide().val("");
				$('#uploadImg_btn').removeClass('notClickBtn').removeAttr('disabled');
		 	}else{
		 		$('.upload_imgBox').show();
			   	$('#video_box').hide();
				//$('#screen_wp').hide().val("");
				$('#sucess_img').attr({"src":' '});
		 		$('#uploadImg_btn').addClass('notClickBtn').attr('disabled','disabled');
		 	}



	   	 	isfileUp=true;
		 }


		 //监听file上传状态
		 $('#avatar').on('change', function(){
		 	if($(this).val().length>0){
		 		//获取后缀名
				var suffix=$(this).val().split('.').splice(-1)[0];
				//获取文件
				var file = this.files[0];

				upload(suffix,file);
		 	}else{

		 		notUpload();
		 	}
		 })



		//监听input欢迎语
		// $('#screen_welcome').on('keyup blur',function(){
		// 	if($.trim($(this).val())==''){
		// 		$('#uploadImg_btn').addClass('notClickBtn').attr('disabled','disabled');
		// 		return ;
		// 	}


		// 	if($(this).val()!=welcome_content){
		// 		$('#uploadImg_btn').removeClass('notClickBtn').removeAttr('disabled');
		// 	}else{
		// 		if(!isfileUp){
		// 			$('#uploadImg_btn').addClass('notClickBtn').attr('disabled','disabled');
		// 		}
		// 	}
		// });

	}
	mainJs.prototype.regNumber=function(){//手机验证

		$(".p_number").keyup(function(){
					var phone = $(this).val();
					$(this).val(phone.replace(/[^(\d|\-)]/g,''));
				})

	}

	mainJs.prototype.showActive=function(){
		var src = $("#mainFrame").attr("src");
		var lis = $(".leftbar_ul>li>ul>li>span");
		for(var i=0;i<lis.length;i++){
			var lisStr = lis.eq(i).attr("href");
			if(lisStr==src){
				lis.eq(i).parent().parent().show();
				lis.eq(i).parent().parent().parent().attr("class","folder-open");
				lis.eq(i).parent().css({"background":"#e6f7ff","borderRight":"2px solid #1890ff"});
				lis.eq(i).css({"color":"#1890ff"});
				lis.eq(i).attr("class","active");
			}
		}
	}

	/**
	 * 分页处理，包括跳转，搜索跳转
	 * total-总页数 cur-当前页 tips-提示信息 hrefn-搜索之后的分页函数
	 */
	mainJs.prototype.page = function(total,cur,tips,hrefn){
		var _this = this;
		total = parseInt(total);

		if(total==0 || isNaN(total)){
			total=1;
			cur=1;
		}

		$("#page").bootstrapPaginator({
	      	currentPage: cur,
	      	totalPages: total,
	      	bootstrapMajorVersion: 3,
	      	size: "small",
	      	onPageClicked: function(e,originalEvent,type,page){
	      		if(page != cur){
	      			hrefn(page);
	      		}
	      	}
	    });

	    //跳转
	    $('#page_sBtn').click(function(){
	    	var num = parseInt($('#page_skip').val());
            var total = $('#page_num').text();

            if(isNaN(num) || num == ''){
                // _this.tiperr(tips[0]);
                alert(tips[0]);
                $('#page_skip').val('');
                return ;
            }else if(num > total || num <= 0){
            	// _this.tiperr(tips[1]);
            	alert(tips[1]);
                $('#page_skip').val('');
                return ;
            }else if(num == cur){
            	// _this.tiperr(tips[2]);\
            	alert(tips[2]);
            	$('#page_skip').val('');
            	return ;
            }

            $("#page").bootstrapPaginator("show",num);

            hrefn(num);
	    });

	    //支持回车
	    $('#page_skip').keydown(function(event) {
            if(event.keyCode==13){
                $('#page_sBtn').trigger('click');
            }
        });

        //如果只有一页就隐藏分页
        if(total == 1){
        	$('.tab_footer').css("visibility","hidden");
        }else{
        	$('.tab_footer').css("visibility","visible");
        }
	}

	/**
	 * alert提示 ok
	 */
	mainJs.prototype.tipok = function(msg){
		$('.a_tip').stop(true,true).fadeIn().addClass('a_tipSucc').removeClass('a_tipErr').text(msg).css({
			'top':parseInt($('.a_tip').height()+$(window.parent).scrollTop())});

		setTimeout(function(){
			$('.a_tip').fadeOut();
		},2000);
	}

	 /**
	 * alert提示 err
	 */
	mainJs.prototype.tiperr = function(msg){
		$('.a_tip').stop(true,true).fadeIn().addClass('a_tipErr').removeClass('a_tipSucc').text(msg).css({
			'top':parseInt($('.a_tip').height()+$(window.parent).scrollTop())});

		setTimeout(function(){
			$('.a_tip').fadeOut();
		},2500);
	}

    /**
	 * 删除提示
	 */
    mainJs.prototype.delTip = function(msg){
    	$('.del').click(function(){
    		return confirm(msg);
    	})
    }

	/**
	 * 搜索条件没查询到
	 */
	mainJs.prototype.notFind = function(msg){
		//未搜索到信息，并隐藏分页
		var str = $.trim($('#tb-1 > tbody').html());
		var tdlen = $('.tab_rows1 > th').length;

		if(str == ""){
			$('#tb-1 > tbody').append('<tr><td style="text-align:center;" colspan='+tdlen+'>'+msg+'</td></tr>');
			$('#page').hide();
			$('#page_extra').hide();
		}else{
			$('#page').show();
			$('#page_extra').show();
		}
	}

	/**
	 * del 单条信息删除
	 */
	mainJs.prototype.del = function(msg){
		$(".tab_del").click(function() {
	        return confirm(msg);
	    });
	}

	/**
	 * 用户操作记录 轨迹
	 */
	mainJs.prototype.footprint = function(ajaxUrl){
		$('.fprint_btn').click(function(){
			$('#maskbody').show();
			var zstype = $(this).data("type");
			$.ajax({
			   type: "POST",
			   url: ajaxUrl,
			   data: {"id":$(this).data("id")},
			   success: function(msg){
			   		$('.mask_ul').html('');
			   		if(msg){
					    if(zstype == "audit"){
							if(msg.auditname && msg.auditname != "") {
					   			var arr = msg.auditname.split('|');
					   			arr.length = arr.length-1;

				   				var html = "";

				   				for(var i=0;i<arr.length;i++){
				   					html += '<li>'+arr[i]+'</li>';
				   				}
				   				$('.mask_ul').append(html);
							}else{
								$('.mask_ul').append("");
							}
						}else if(zstype == "sync"){
							if(msg.svmsg && msg.svmsg != "") {
					   			var arr = msg.svmsg.split('|');
					   			arr.length = arr.length-1;

				   				var html = "";

				   				for(var i=0;i<arr.length;i++){
				   					html += '<li>'+arr[i]+'</li>';
				   				}
				   				$('.mask_ul').append(html);
							}else{
								$('.mask_ul').append("");
							}
						}
			   		}else{
			   			console.log("查询失败");
			   		}
			   }
			});

			//跟随滚动和窗口改变 定制弹出框的高度
			$('.simplemodal-close').click(function(){
				$('#maskbody').hide();
			});

				function alrmove(){
				$('#mask').stop(true).animate({
					'left':parseInt(($(window.parent).width()-$('#mask').width()-200)/2+$(window.parent).scrollLeft()),
					'top':parseInt(($(window.parent).height()-$('#mask').height()-100)/2+$(window.parent).scrollTop())
				});
			}

			alrmove();

			$(window.parent).on('resize scroll',alrmove);
		});
	}

	/**
	 * 闲聊的审核，批量审核
	 */
	mainJs.prototype.thirdchat = function(ajaxUrl,tips){
		$('.review_a').each(function(){
			if($(this).attr("data-status") == 'false'){
				$(this).attr("data-status",'no');
			}else{
				$(this).attr("data-status",'yes');
			}
		});

		this.review(ajaxUrl,tips,"chat");
	}

	/**
	 * 快速审核，批量审核
	 * tips-i18n提示信息
	 */
	mainJs.prototype.review = function(ajaxUrl,tips,chat){
		var that = this;
		var reviewlen = $('.review_cball').length; //判断批量审核checkbox是否存在

		//单点快速审核
		$('.review_a').each(function(){
			if($(this).attr("data-status") == "no"){
				$(this).addClass('review_bgoff');
			}else{
				$(this).addClass('review_bgon');
			}
		}).on('click',function(){
			var _this =$(this);

			if(confirm(tips[0])){
				var status = "";

				if($(this).attr("data-status") == "no"){
					status = "yes";
				}else{
					status = "no";
				};

				ajaxreview(status,_this);
			}
		});

		//批量审核
		if(reviewlen){
			$('.review_cball').click(function(){
				var _this = this;

				$('.review_cbox').each(function(){
					if($(this).attr('disabled')==undefined){
						$(this).prop({'checked':_this.checked});
					}
				});
			});

			$('#batchAudit').click(function(){
				var len = 0 ;
				$('.review_cbox').each(function(index){
					if($(this).attr('disabled')==undefined && $(this).prop('checked')==true){
						len++;
						var _this = $('.review_a').eq(index);

						//ajax批量审核
						ajaxreview("yes",_this);
					}
				});

				if(len == 0){
					that.tiperr(tips[4]);
				}
			});
		}


		//ajax审核内部函数
		function ajaxreview(status,_this){
			var info = {"status":status,"id":_this.data("id")};

			if(chat){
				if(status=='yes'){
					status =true;
				}else{
					status =false;
				}

				var an = _this.parent().prev().text(),
				    qu = _this.parent().prev().prev().text();
				    info = {"status":status,"id":_this.data("id")+"|"+md5(qu+an)};

				// console.log(info,status);
			}

			var findQaStr = ajaxUrl.search("qaitemagent");

			$.ajax({
			    type: "POST",
			   	url: ajaxUrl,
			   	data:info,
			   	beforeSend: function(){
			   		_this.hide();
			   		var html = '<img src="/static/Images/loading4.gif" class="review_load" />';

			   		_this.hide().parent().append(html);
			   	},
			   	success: function(msg){
					if(msg){
			   			// 需要提示相应错误信息
			   			if (typeof(msg) == "string" && msg.substr(0,6) === "\"error:"){
							that.tiperr(msg.substr(6));
							$('#s'+_this.data("id")).html(tips[6]);
						}else if(_this.attr("data-status") == "no"){
		   					_this.addClass('review_bgon').removeClass('review_bgoff').attr("data-status","yes");

		   					if(reviewlen){
		   						_this.parents('tr').find('.review_cbox').prop({'disabled':true,'checked':true});
		   					}

		   					that.tipok(tips[1]);
						}else{
							_this.addClass('review_bgoff').removeClass('review_bgon').attr("data-status","no");

							if(reviewlen){
		   						_this.parents('tr').find('.review_cbox').prop({'disabled':false,'checked':false});
		   					}

							that.tipok(tips[2]);
						}
			   		}else{
			   			that.tiperr(tips[3]);
						$('#s'+_this.data("id")).html(tips[6]);
					}

					//审核成功再去查询sv同步状态
					// if(findQaStr !== 1){
					// 	that.svupdate(_this.data("id"),tips);
					// } else{
					// 	$('#s'+_this.data("id")).html(tips[5]);
					// }
					that.svupdate(_this.data("id"),tips);
				},
			   	complete: function(){
			   		_this.show().next().remove();
			   	}
			});
		}
	}

	mainJs.prototype.batchReview = function(ajaxUrl,tips,chat){//mark:limei 批量审核
           var that = this;
		var reviewlen = $('.review_cball').length; //判断批量审核checkbox是否存在

		//单点快速审核
		$('.review_a').each(function(){
			if($(this).attr("data-status") == "no"){
				$(this).addClass('review_bgoff');
			}else{
				$(this).addClass('review_bgon');
			}
		}).on('click',function(){
			var _this =$(this);

			if(confirm(tips[0])){
				var status = "";

				if($(this).attr("data-status") == "no"){
					status = "yes";
				}else{
					status = "no";
				};

				ajaxreview(status,_this);
			}
		});

		//批量审核
		if(reviewlen){
			$('.review_cball').click(function(){
				var _this = this;
				if($(this).hasClass('checkAll')){
					$('.review_cbox').prop({'checked':!_this.checked});
				}else{
					$('.review_cbox').prop({'checked':_this.checked});
				}

			});

			$('#batchAudit').click(function(){
				var len = 0 ;
				$('.review_cbox').each(function(index){
					if($(this).attr('status')=='no' && $(this).prop('checked')==true){
						len++;
						var _this = $('.review_a').eq(index);
						//ajax批量审核
						ajaxreview("yes",_this);
					}
				});

				if(len == 0){
					that.tiperr(tips[4]);
				}
			});
		}


		//ajax审核内部函数
		function ajaxreview(status,_this){
			var info = {"status":status,"id":_this.data("id")};

			if(chat){
				if(status=='yes'){
					status =true;
				}else{
					status =false;
				}

				var an = _this.parent().prev().text(),
				    qu = _this.parent().prev().prev().text();
				    info = {"status":status,"id":_this.data("id")+"|"+md5(qu+an)};

			}

			var findQaStr = ajaxUrl.search("qaitemagent");

			$.ajax({
			    type: "POST",
			   	url: ajaxUrl,
			   	data:info,
			   	beforeSend: function(){
			   		_this.hide();
			   		var html = '<img src="/static/Images/loading4.gif" class="review_load" />';

			   		_this.hide().parent().append(html);
			   	},
			   	success: function(msg){
					if(msg){
						// 需要提示相应错误信息
						if (typeof(msg)=="string" && msg.substr(0,6) === "error:") {
							that.tiperr(msg.substr(6));
							$('#s' + _this.data("id")).html(tips[6]);
						}else if(_this.attr("data-status") == "no"){
		   					_this.addClass('review_bgon').removeClass('review_bgoff').attr("data-status","yes");

		   					that.tipok(tips[1]);
						}else{
							_this.addClass('review_bgoff').removeClass('review_bgon').attr("data-status","no");

							that.tipok(tips[2]);
						}
			   		}else{
			   			that.tiperr(tips[3]);
						$('#s'+_this.data("id")).html(tips[6]);
					}

					//审核成功再去查询sv同步状态
					// if(findQaStr !== 1){
					// 	that.svupdate(_this.data("id"),tips);
					// } else{
					// 	$('#s'+_this.data("id")).html(tips[5]);
					// }
					that.svupdate(_this.data("id"),tips);
				},
			   	complete: function(){
			   		_this.show().next().remove();
			   	}
			});
		}
	}

	/**
	 * 与SmartVoice同步状态
	 */
	mainJs.prototype.svupdate = function(id,tips){
		var _this = this;

		if($('#s'+id).length > 0){
			
			var html = '<img src="/static/Images/loading4.gif" class="review_load" />';
			$('#s'+id).html(html);

			var num = 0
			var syn = 'no'
			function func(){
				$.ajax({
					type: "POST",
					url: _this.ajaxurl,
					data: {"id":id},
					success: function(data){
						num += 1
						syn = data.synstatus
					}
				});

				console.log(num)
				if(num != 4){
					if(syn =='yes'){
						clearInterval(interval );//停止
						$('#s'+id).html(tips[5]);
					}else{
						$('#s'+id).html(html);
					}
				} else {
					if(syn =='yes'){
						clearInterval(interval );//停止
						$('#s'+id).html(tips[5]);
					}else{
						clearInterval(interval );//停止
						$('#s'+id).html(tips[6]);
					}
				}
			} //定时任务

			var interval = setInterval(func, 5000); //启动,func不能使用括号

			// setTimeout(function(){
			// 	$.ajax({
			// 		type: "POST",
			// 		url: _this.ajaxurl,
			// 		data: {"id":id},
			// 		success: function(data){
			// 			if(data.synstatus === "yes"){
			// 				$('#s'+id).html(tips[5]);
			// 			}else{
			// 				$('#s'+id).html(tips[6]);
			// 			}
			// 			// console.log(data);
			// 		}
			// 	});
			// },5000);
		}
	}
	mainJs.prototype.progressFn = function(cent){
		// 获取最外层的div
		var oDiv1 = document.getElementById('progressBox');
					// 获取内层进度条的div
		var oDiv2 = document.getElementById('progressBar');
		// 获取内层文字发生变化时的div
		var oDiv3 = document.getElementById('progressText');
	
		// 获取总进度条的宽度
		var allWidth = parseInt(getStyle(oDiv1, 'width'));
	
		// 设定内层两个div的文字内容一样
		oDiv2.innerHTML = cent + '%';
		oDiv3.innerHTML = cent + '%';
	
		// 修改clip的的宽度值
		oDiv2.style.clip = 'rect(0px, ' + cent / 100 * allWidth + 'px, 40px, 0px)';
	
		// 获取当前元素的属性值
		function getStyle(obj, attr) {
			// 兼容IE
			if (obj.currentStyle) {
				return obj.currentStyle[attr];
			}else {
				// 第二个参数为false是通用的写法，目的是为了兼容老版本
				return getComputedStyle(obj, false)[attr];
			}
		}
	};
	// 下载进度条
	mainJs.prototype.exportFun = function(url,ids,name){
		var that = this
		if(ids){
			var xhr = new XMLHttpRequest();
			xhr.open('GET',url,true);
			xhr.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
			xhr.responseType = "blob";
			xhr.onprogress = function (event) {
				if(event.lengthComputable){
					$('#progressBox').show()
					var iNow = 0;
					// 设定定时器
					var timer = setInterval(function () {
						// 如果当前的值为100
						console.log(iNow,event.total)
						if (iNow === 100) {
							// 清除定时器
							clearInterval(timer);
							$('#progressBox').hide()
						}else {
							// 将当前状态值累加1
							iNow += 1;
							// 调用执行状态的函数,传入状态值
							that.progressFn(iNow);
						}
	
					}, 30);
					
				}        
				
			};
			xhr.onload = function (oEvent) {
				if (xhr.readyState === 4 && xhr.status === 200) {
				var blob = new Blob([xhr.response],{
				type: "application/vnd.ms-excel"}
				);
				var csvUrl = URL.createObjectURL(blob);
				var link = document.createElement('a');
				link.href = csvUrl;
				link.download = name;
				link.click();   
				}
			}
			xhr.send();
		}else{
			window.open(url)
		}
	}

	var m=new mainJs();

	return m;

})();

(typeof jQuery != "undefined") && (function($){
	if (typeof $.validator == "undefined"){
		return
	}
	$.validator.addMethod("isMobile", function(value, element) {
		var length = value.length;
		var mobile = /^(13[0-9]{9})|(18[0-9]{9})|(14[0-9]{9})|(17[0-9]{9})|(15[0-9]{9})$/;
		return this.optional(element) || (length == 11 && mobile.test(value));
	   }, "请正确填写手机号码");
})(jQuery)
