<template>
	<div>
		<Comment/>
		<CommentPagination/>
	</div>
</template>

<script setup>
	import {watch, onMounted} from 'vue'
	import {useRoute} from 'vue-router'
	import Comment from './Comment.vue'; import CommentPagination from './CommentPagination.vue'; import {useStore} from '@/store'
	import {SET_COMMENT_QUERY_PAGE, SET_COMMENT_QUERY_BLOG_ID, SET_COMMENT_QUERY_PAGE_NUM, SET_PARENT_COMMENT_ID} from '@/store/mutations-types'
	defineOptions({name: 'CommentList'})
	const props = defineProps({page: {type: Number, required: true}, blogId: Number}); const store = useStore(); const route = useRoute()
	function init() { store[SET_PARENT_COMMENT_ID](-1); store[SET_COMMENT_QUERY_PAGE](props.page); store[SET_COMMENT_QUERY_BLOG_ID](props.blogId); store[SET_COMMENT_QUERY_PAGE_NUM](1); store.getCommentList() }
	onMounted(init); watch(() => route.path, init)
</script>

<style scoped>

</style>
