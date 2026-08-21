<template>
	<!--标签云-->
	<div class="ui segments m-box sidebar-panel">
		<div class="ui secondary segment"><i class="tags icon"></i>标签云</div>
		<div class="ui yellow segment m-padding-small tag-cloud">
			<router-link
				:to="`/tag/${tag.name}`"
				class="tag-pill tag-color"
				:class="tagColorClass(tag)"
				v-for="(tag,index) in tagList"
				:key="index"
			>
				{{ tag.name }}
			</router-link>
		</div>
	</div>
</template>

<script>
	import { tagColorClass } from '@/util/tagColor'

	export default {
		name: "Tags",
		props: {
			tagList: {
				type: Array,
				required: true
			},
		},
		methods: {
			tagColorClass,
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
		border: 1px solid var(--tag-border);
		background: var(--tag-bg);
		color: var(--tag-foreground) !important;
		font-size: 12px;
		font-weight: 700;
		padding: 7px 10px;
		transition: background .2s ease, border-color .2s ease, color .2s ease, transform .2s ease;
	}

	.tag-pill:hover {
		background: var(--tag-hover-bg);
		color: var(--tag-foreground) !important;
		transform: translateY(-1px);
	}
</style>
