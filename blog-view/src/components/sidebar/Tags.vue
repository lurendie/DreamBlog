<template>
	<!--标签云-->
	<div class="ui segments m-box sidebar-panel">
		<div class="ui secondary segment"><i class="tags icon"></i>标签云</div>
		<div class="ui yellow segment m-padding-small tag-cloud">
			<router-link :to="`/tag/${tag.name}`" class="tag-pill" v-for="(tag,index) in tagList" :key="index">
				{{ tag.name }}
			</router-link>
		</div>
	</div>
</template>

<script>
	export default {
		name: "Tags",
		props: {
			tagList: {
				type: Array,
				required: true
			},
		},
	}
</script>

<style scoped>
	.secondary.segment {
		padding: 10px;
	}

	.m-padding-small {
		padding: 12px !important;
	}

	.sidebar-panel {
		overflow: hidden;
	}

	.tag-cloud {
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
	}

	.tag-pill {
		border-radius: 999px;
		background: #eef7f6;
		color: #0f766e !important;
		font-size: 12px;
		font-weight: 700;
		padding: 7px 10px;
		transition: background .2s ease, color .2s ease;
	}

	.tag-pill:hover {
		background: #0f766e;
		color: #fff !important;
	}
</style>
