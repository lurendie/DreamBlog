<template>
	<div>
		<div class="ui top segment" style="text-align: center">
			<h2 class="m-text-500">标签 {{ tagName }} 下的文章</h2>
		</div>
		<BlogList :getBlogList="getBlogList" :blogList="blogList" :totalPage="totalPage"/>
	</div>
</template>

<script setup>
	import {ref, computed, watch, onMounted} from 'vue'
	import {useRoute} from 'vue-router'
	import {storeToRefs} from 'pinia'
	import {ElMessage} from 'element-plus'
	import BlogList from '@/components/blog/BlogList.vue'; import {getBlogListByTagName} from '@/api/tag'; import {updateSeo} from '@/util/seo'; import {useStore} from '@/store'
	defineOptions({name: 'Tag'})
	const route = useRoute(); const {siteInfo} = storeToRefs(useStore()); const blogList = ref([]); const totalPage = ref(0); const tagName = computed(() => route.params.name)
	async function getBlogList(pageNum) { try { const res = await getBlogListByTagName(tagName.value, pageNum); if (res.code === 200) { blogList.value = res.data.list; totalPage.value = res.data.totalPage; updateSeo({title: `标签：${tagName.value}`, description: siteInfo.value?.siteDescription || '', keywords: siteInfo.value?.siteKeywords || '', path: route.fullPath}) } else ElMessage.error(res.msg) } catch (_) { ElMessage.error('请求失败') } }
	onMounted(getBlogList); watch(() => route.fullPath, () => { if (route.name === 'tag') getBlogList() })
</script>

<style scoped>

</style>
