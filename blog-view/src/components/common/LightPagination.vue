<template>
	<nav v-if="pageCount > 1" class="light-pagination" aria-label="分页">
		<button type="button" :disabled="currentPage <= 1" @click="changePage(currentPage - 1)">
			上一页
		</button>
		<button
			v-for="page in visiblePages"
			:key="page.key"
			type="button"
			:class="{'is-active': page.value === currentPage, 'is-gap': page.gap}"
			:disabled="page.gap"
			@click="!page.gap && changePage(page.value)"
		>
			{{ page.label }}
		</button>
		<button type="button" :disabled="currentPage >= pageCount" @click="changePage(currentPage + 1)">
			下一页
		</button>
	</nav>
</template>

<script>
	export default {
		name: "LightPagination",
		props: {
			currentPage: {
				type: Number,
				required: true
			},
			pageCount: {
				type: Number,
				required: true
			},
			siblingCount: {
				type: Number,
				default: 1
			}
		},
		emits: ['current-change'],
		computed: {
			visiblePages() {
				const pageCount = Math.max(0, Number(this.pageCount) || 0)
				const currentPage = Math.min(Math.max(1, Number(this.currentPage) || 1), pageCount || 1)
				const siblingCount = Math.max(1, Number(this.siblingCount) || 1)
				const pages = []

				const addPage = page => {
					pages.push({key: `page-${page}`, value: page, label: String(page), gap: false})
				}
				const addGap = key => {
					pages.push({key, value: null, label: '...', gap: true})
				}

				if (pageCount <= 7) {
					for (let page = 1; page <= pageCount; page++) {
						addPage(page)
					}
					return pages
				}

				addPage(1)
				const start = Math.max(2, currentPage - siblingCount)
				const end = Math.min(pageCount - 1, currentPage + siblingCount)

				if (start > 2) {
					addGap('start-gap')
				}
				for (let page = start; page <= end; page++) {
					addPage(page)
				}
				if (end < pageCount - 1) {
					addGap('end-gap')
				}
				addPage(pageCount)
				return pages
			}
		},
		methods: {
			changePage(page) {
				const nextPage = Math.min(Math.max(1, page), this.pageCount)
				if (nextPage !== this.currentPage) {
					this.$emit('current-change', nextPage)
				}
			}
		}
	}
</script>

<style scoped>
	.light-pagination {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: 8px;
		margin-top: 1.5rem;
	}

	.light-pagination button {
		min-width: 38px;
		height: 36px;
		padding: 0 12px;
		border: 1px solid rgba(148, 163, 184, 0.22);
		border-radius: 8px;
		background: #ffffff;
		color: #475569;
		font-size: 14px;
		font-weight: 700;
		cursor: pointer;
	}

	.light-pagination button:hover:not(:disabled) {
		border-color: rgba(15, 118, 110, 0.4);
		color: #0f766e;
	}

	.light-pagination button.is-active {
		border-color: #0f766e;
		background: #0f766e;
		color: #ffffff;
	}

	.light-pagination button:disabled {
		cursor: default;
		opacity: 0.45;
	}

	.light-pagination button.is-gap {
		border-color: transparent;
		background: transparent;
		opacity: 1;
	}

	@media screen and (max-width: 767px) {
		.light-pagination {
			gap: 6px;
			margin-top: 1.25rem;
		}

		.light-pagination button {
			min-width: 34px;
			height: 34px;
			padding: 0 10px;
			font-size: 13px;
		}
	}
</style>
