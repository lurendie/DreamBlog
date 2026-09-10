<template>
	<div>
		<BlogList :getBlogList="getBlogList" :blogList="blogList" :totalPage="totalPage"/>
	</div>
</template>

<script setup>
	import {ref, onMounted} from 'vue'
	import {useRoute, onBeforeRouteUpdate} from 'vue-router'
	import BlogList from '@/components/blog/BlogList.vue'
	import {getBlogList as fetchBlogList} from '@/api/home'
	import {useStore} from '@/store'
	import {SET_IS_BLOG_TO_HOME} from '@/store/mutations-types'
	import {ElMessage} from 'element-plus'

	defineOptions({name: 'Home'})
	const store = useStore(); const route = useRoute(); const blogList = ref([]); const totalPage = ref(0); const getBlogListFinish = ref(false)
	async function getBlogList(pageNum) {
		try { const res = await fetchBlogList(pageNum); if (res.code === 200) { blogList.value = res.data.list; totalPage.value = res.data.totalPage; getBlogListFinish.value = true } else ElMessage.error(res.msg) } catch (_) { ElMessage.error('请求失败') }
	}
	function handleEnter(fromName) { if (fromName !== 'blog') { store[SET_IS_BLOG_TO_HOME](false); getBlogList() } else { if (!getBlogListFinish.value) getBlogList(); store[SET_IS_BLOG_TO_HOME](true) } }
	onMounted(() => handleEnter(route.name === 'home' ? '' : route.name))
	onBeforeRouteUpdate((to, from) => { if (to.name === 'home') handleEnter(from.name) })
</script>

<style scoped>

</style>
