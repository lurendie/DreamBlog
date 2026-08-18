<template>
	<div class="metric-card" :class="`metric-card--${accent}`">
		<div class="metric-card__content">
			<span class="metric-card__label">{{ title }}</span>
			<strong class="metric-card__value">{{ value }}</strong>
			<span v-if="description" class="metric-card__description">{{ description }}</span>
		</div>
		<div class="metric-card__icon">
			<SvgIcon :icon-class="icon" class-name="metric-card__svg" />
		</div>
	</div>
</template>

<script>
	import SvgIcon from '@/components/SvgIcon'

	export default {
		name: 'MetricCard',
		components: {SvgIcon},
		props: {
			title: {
				type: String,
				required: true
			},
			value: {
				type: [String, Number],
				default: 0
			},
			description: {
				type: String,
				default: ''
			},
			icon: {
				type: String,
				required: true
			},
			accent: {
				type: String,
				default: 'blue'
			}
		}
	}
</script>

<style lang="scss" scoped>
	.metric-card {
		position: relative;
		display: flex;
		min-height: 128px;
		align-items: center;
		justify-content: space-between;
		gap: 18px;
		overflow: hidden;
		border: 1px solid #e4e9f2;
		border-radius: 8px;
		background: #fff;
		padding: 22px;
		box-shadow: 0 14px 34px rgba(30, 41, 59, 0.06);
	}

	.metric-card:before {
		content: '';
		position: absolute;
		inset: 0 auto 0 0;
		width: 4px;
		background: var(--metric-accent);
	}

	.metric-card__content {
		position: relative;
		z-index: 1;
		display: flex;
		min-width: 0;
		flex-direction: column;
	}

	.metric-card__label {
		color: #64748b;
		font-size: 13px;
		font-weight: 650;
	}

	.metric-card__value {
		margin-top: 8px;
		color: #172033;
		font-size: 30px;
		font-weight: 700;
		line-height: 1;
	}

	.metric-card__description {
		margin-top: 10px;
		color: #8a97aa;
		font-size: 12px;
	}

	.metric-card__icon {
		display: grid;
		width: 52px;
		height: 52px;
		flex: 0 0 auto;
		place-items: center;
		border-radius: 50%;
		background: var(--metric-soft);
		color: var(--metric-accent);
	}

	.metric-card__svg {
		width: 26px;
		height: 26px;
	}

	.metric-card--blue {
		--metric-accent: #3b82f6;
		--metric-soft: #dbeafe;
	}

	.metric-card--teal {
		--metric-accent: #14b8a6;
		--metric-soft: #ccfbf1;
	}

	.metric-card--amber {
		--metric-accent: #f59e0b;
		--metric-soft: #fef3c7;
	}

	.metric-card--violet {
		--metric-accent: #8b5cf6;
		--metric-soft: #ede9fe;
	}
</style>
