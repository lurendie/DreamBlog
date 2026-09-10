import { createPinia, defineStore } from 'pinia'
import initialState from './state'
import actions from './actions'
import mutations from './mutations'
import {
	SAVE_SITE_INFO,
	SAVE_INTRODUCTION,
	SAVE_COMMENT_RESULT,
	SET_COMMENT_QUERY_PAGE_NUM,
	SET_PARENT_COMMENT_ID,
	RESET_COMMENT_FORM,
	RESTORE_COMMENT_FORM,
	SET_COMMENT_QUERY_PAGE,
	SET_COMMENT_QUERY_BLOG_ID,
	SET_IS_BLOG_RENDER_COMPLETE,
	SET_BLOG_PASSWORD_DIALOG_VISIBLE,
	SET_BLOG_PASSWORD_FORM,
	SET_FOCUS_MODE,
	SET_IS_BLOG_TO_HOME,
	SAVE_CLIENT_SIZE,
} from './mutations-types'

export const useStore = defineStore('main', {
	state: () => ({
		...initialState,
		commentQuery: {...initialState.commentQuery},
		commentForm: {...initialState.commentForm},
		blogPasswordForm: {...initialState.blogPasswordForm},
		clientSize: {...initialState.clientSize},
	}),
	actions: {
		// Keep mutation names as actions so existing callers can migrate incrementally.
		[SAVE_SITE_INFO](value) { mutations[SAVE_SITE_INFO](this, value) },
		[SAVE_INTRODUCTION](value) { mutations[SAVE_INTRODUCTION](this, value) },
		[SAVE_COMMENT_RESULT](value) { mutations[SAVE_COMMENT_RESULT](this, value) },
		[SET_COMMENT_QUERY_PAGE](value) { mutations[SET_COMMENT_QUERY_PAGE](this, value) },
		[SET_COMMENT_QUERY_BLOG_ID](value) { mutations[SET_COMMENT_QUERY_BLOG_ID](this, value) },
		[SET_COMMENT_QUERY_PAGE_NUM](value) { mutations[SET_COMMENT_QUERY_PAGE_NUM](this, value) },
		[SET_PARENT_COMMENT_ID](value) { mutations[SET_PARENT_COMMENT_ID](this, value) },
		[RESET_COMMENT_FORM]() { mutations[RESET_COMMENT_FORM](this) },
		[RESTORE_COMMENT_FORM]() { mutations[RESTORE_COMMENT_FORM](this) },
		[SET_IS_BLOG_RENDER_COMPLETE](value) { mutations[SET_IS_BLOG_RENDER_COMPLETE](this, value) },
		[SET_BLOG_PASSWORD_DIALOG_VISIBLE](value) { mutations[SET_BLOG_PASSWORD_DIALOG_VISIBLE](this, value) },
		[SET_BLOG_PASSWORD_FORM](value) { mutations[SET_BLOG_PASSWORD_FORM](this, value) },
		[SET_FOCUS_MODE](value) { mutations[SET_FOCUS_MODE](this, value) },
		[SET_IS_BLOG_TO_HOME](value) { mutations[SET_IS_BLOG_TO_HOME](this, value) },
		[SAVE_CLIENT_SIZE](value) { mutations[SAVE_CLIENT_SIZE](this, value) },

		getCommentList() {
			return actions.getCommentList({commit: this.commit.bind(this), dispatch: this.dispatch.bind(this), rootState: this})
		},
		submitCommentForm(token) {
			return actions.submitCommentForm({commit: this.commit.bind(this), dispatch: this.dispatch.bind(this), rootState: this}, token)
		},
		goBlogPage(blog) {
			return actions.goBlogPage({commit: this.commit.bind(this)}, blog)
		},

		// Compatibility methods for Options API components during the migration.
		commit(type, payload) {
			if (typeof this[type] !== 'function') throw new Error(`Unknown store mutation: ${type}`)
			return this[type](payload)
		},
		dispatch(type, payload) {
			if (typeof this[type] !== 'function') throw new Error(`Unknown store action: ${type}`)
			return this[type](payload)
		},
	},
})

export const pinia = createPinia()
const piniaStore = useStore(pinia)

// Existing utility modules and Options API templates read `store.state`; expose a small facade.
const store = {
	state: piniaStore.$state,
	commit: piniaStore.commit.bind(piniaStore),
	dispatch: piniaStore.dispatch.bind(piniaStore),
}

export default store
