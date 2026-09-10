<template>
	<div>
		<div class="ui top segment" style="text-align: center">
			<h2 class="m-text-500">分类 {{ categoryName }} 下的文章</h2>
		</div>
		<BlogList :getBlogList="getBlogList" :blogList="blogList" :totalPage="totalPage"/>
	</div>
</template>

<script setup>
	import {ref, computed, watch, onMounted} from 'vue'; import {useRoute} from 'vue-router'; import {storeToRefs} from 'pinia'; import {ElMessage} from 'element-plus'; import BlogList from '@/components/blog/BlogList.vue'; import {getBlogListByCategoryName} from '@/api/category'; import {updateSeo} from '@/util/seo'; import {useStore} from '@/store'
	defineOptions({name: 'Category'}); const route = useRoute(); const {siteInfo} = storeToRefs(useStore()); const blogList = ref([]); const totalPage = ref(0); const categoryName = computed(() => route.params.name)
	async function getBlogList(pageNum) { try { const res = await getBlogListByCategoryName(categoryName.value, pageNum); if (res.code === 200) { blogList.value = res.data.list; totalPage.value = res.data.totalPage; updateSeo({title: `分类：${categoryName.value}`, description: siteInfo.value?.siteDescription || '', keywords: siteInfo.value?.siteKeywords || '', path: route.fullPath}) } else ElMessage.error(res.msg) } catch (_) { ElMessage.error('请求失败') } }
	onMounted(getBlogList); watch(() => route.fullPath, () => { if (route.name === 'category') getBlogList() })
</script>

<style scoped>

</style>
