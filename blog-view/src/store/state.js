function getInitialClientSize() {
	if (typeof window === 'undefined') {
		return {
			clientHeight: 0,
			clientWidth: 0
		}
	}
	return {
		clientHeight: window.innerHeight,
		clientWidth: window.innerWidth
	}
}

export default {
	siteInfo: '',
	introduction: {
		avatar: '',
		name: '',
		rollText: [],
		favorites: [],
	},
	commentQuery: {
		//用于后端判断该评论所在页面类型(文章、友链、关于我)
		page: 0,
		blogId: null,
		pageNum: 1,
		pageSize: 5
	},
	allComment: 0,
	closeComment: 0,
	commentTotalPage: 0,
	comments: [],
	parentCommentId: -1,
	commentForm: {
		content: '',
		nickname: '',
		email: '',
		website: '',
		notice: true
	},
	//博客文章渲染完成的标记
	isBlogRenderComplete: false,
	//受保护文章密码对话框
	blogPasswordDialogVisible: false,
	blogPasswordForm: {
		blogId: 0,
		password: ''
	},
	//专注模式
	focusMode: false,
	//文章页面路由到首页的标记
	isBlogToHome: false,
	//可视窗口大小
	clientSize: getInitialClientSize()
}
