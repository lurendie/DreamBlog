<template>
	<div class="page-header">
		<div class="page-header__main">
			<div v-if="eyebrow" class="page-header__eyebrow">{{ eyebrow }}</div>
			<h1 class="page-header__title">{{ title }}</h1>
			<p v-if="description" class="page-header__description">{{ description }}</p>
		</div>
		<div v-if="$slots.actions" class="page-header__actions">
			<slot name="actions" />
		</div>
	</div>
</template>

<script>
	export default {
		name: 'PageHeader',
		props: {
			title: {
				type: String,
				required: true
			},
			description: {
				type: String,
				default: ''
			},
			eyebrow: {
				type: String,
				default: ''
			}
		}
	}
</script>

<style lang="scss" scoped>
	.page-header {
		display: flex;
		align-items: flex-end;
		justify-content: space-between;
		gap: 20px;
		margin-bottom: 20px;
	}

	.page-header__main {
		min-width: 0;
	}

	.page-header__eyebrow {
		margin-bottom: 6px;
		color: #64748b;
		font-size: 12px;
		font-weight: 700;
		letter-spacing: 0;
	}

	.page-header__title {
		margin: 0;
		color: #172033;
		font-size: 26px;
		font-weight: 650;
		line-height: 1.25;
	}

	.page-header__description {
		margin: 8px 0 0;
		color: #64748b;
		font-size: 14px;
		line-height: 1.6;
	}

	.page-header__actions {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		justify-content: flex-end;
		gap: 10px;
	}

	@media screen and (max-width: 768px) {
		.page-header {
			display: block;
		}

		.page-header__actions {
			justify-content: flex-start;
			margin-top: 14px;
		}

		.page-header__title {
			font-size: 22px;
		}
	}
</style>
