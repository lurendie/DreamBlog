<template>
	<section class="chart-panel" :style="{ minHeight: `${height}px` }">
		<header class="chart-panel__header">
			<div>
				<h2 class="chart-panel__title">{{ title }}</h2>
				<p v-if="description" class="chart-panel__description">{{ description }}</p>
			</div>
			<div v-if="tabs.length" class="chart-panel__tabs">
				<button
					v-for="tab in tabs"
					:key="tab.value"
					type="button"
					class="chart-panel__tab"
					:class="{ 'is-active': activeTab === tab.value }"
					@click="$emit('update:activeTab', tab.value)"
				>
					{{ tab.label }}
				</button>
			</div>
		</header>
		<div class="chart-panel__body">
			<slot />
		</div>
	</section>
</template>

<script>
	export default {
		name: 'ChartPanel',
		props: {
			title: {
				type: String,
				required: true
			},
			description: {
				type: String,
				default: ''
			},
			height: {
				type: Number,
				default: 420
			},
			tabs: {
				type: Array,
				default: () => []
			},
			activeTab: {
				type: String,
				default: ''
			}
		},
		emits: ['update:activeTab']
	}
</script>

<style lang="scss" scoped>
	.chart-panel {
		display: flex;
		flex-direction: column;
		overflow: hidden;
		border: 1px solid rgba(148, 163, 184, 0.18);
		border-radius: 8px;
		background: #fff;
		box-shadow: 0 18px 45px rgba(15, 23, 42, 0.08);
	}

	.chart-panel__header {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 16px;
		border-bottom: 1px solid rgba(148, 163, 184, 0.14);
		padding: 18px 20px 14px;
	}

	.chart-panel__title {
		margin: 0;
		color: #172033;
		font-size: 16px;
		font-weight: 650;
		line-height: 1.4;
	}

	.chart-panel__description {
		margin: 4px 0 0;
		color: #8a97aa;
		font-size: 12px;
	}

	.chart-panel__body {
		min-height: 0;
		flex: 1;
		padding: 16px;
	}

	.chart-panel__tabs {
		display: inline-flex;
		flex-wrap: wrap;
		gap: 6px;
		padding: 4px;
		border-radius: 999px;
		background: #f8fafc;
	}

	.chart-panel__tab {
		border: 0;
		border-radius: 999px;
		background: transparent;
		color: #64748b;
		font-size: 12px;
		font-weight: 650;
		padding: 7px 12px;
		cursor: pointer;
		transition: background-color .2s, color .2s, box-shadow .2s;
	}

	.chart-panel__tab:hover {
		color: #334155;
	}

	.chart-panel__tab.is-active {
		background: #fff;
		color: #0f766e;
		box-shadow: 0 1px 4px rgba(15, 23, 42, 0.08);
	}

	@media screen and (max-width: 768px) {
		.chart-panel__header {
			display: block;
		}

		.chart-panel__tabs {
			margin-top: 12px;
		}
	}
</style>
