<template>
	<!--评论分页-->
	<LightPagination
		class="pagination"
		:current-page="commentQuery.pageNum"
		:page-count="commentTotalPage"
		@current-change="handleCurrentChange"
	/>
</template>

<script setup>
	import {storeToRefs} from 'pinia'
	import {useStore} from '@/store'
	import {SET_COMMENT_QUERY_PAGE_NUM, SET_PARENT_COMMENT_ID} from "@/store/mutations-types"
	import LightPagination from "@/components/common/LightPagination.vue"

	defineOptions({name: 'CommentPagination'})
	const store = useStore()
	const {commentQuery, commentTotalPage} = storeToRefs(store)
	function handleCurrentChange(newPage) {
		store[SET_COMMENT_QUERY_PAGE_NUM](newPage)
		store[SET_PARENT_COMMENT_ID](-1)
		store.getCommentList()
	}
</script>

<style scoped>
	.pagination {
		margin-top: 2em;
		text-align: center;
	}

	@media screen and (max-width: 767px) {
		.pagination {
			margin-top: 1.25rem;
		}
	}
</style>
