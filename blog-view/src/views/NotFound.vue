<template>
	<main class="not-found">
		<div class="not-found__content">
			<p class="not-found__code" aria-label="404">404</p>
			<p class="not-found__eyebrow">页面不存在</p>
			<h1>找不到你要访问的页面</h1>
			<p class="not-found__description">
				页面可能已被移动、删除，或者地址输入有误。
			</p>
			<div class="not-found__actions">
				<router-link to="/home" class="ui primary button">
					<i class="home icon"></i>
					返回首页
				</router-link>
				<button type="button" class="ui basic button" @click="goBack">
					<i class="arrow left icon"></i>
					返回上一页
				</button>
			</div>
		</div>
	</main>
</template>

<script>
	export default {
		name: 'NotFound',
		methods: {
			goBack() {
				if (window.history.length > 1) {
					this.$router.back()
					return
				}
				this.$router.push('/home')
			}
		}
	}
</script>

<style scoped>
	.not-found {
		box-sizing: border-box;
		display: flex;
		min-height: 100vh;
		align-items: center;
		justify-content: center;
		padding: 32px 20px;
		background:
			linear-gradient(135deg, rgba(37, 204, 247, 0.1), transparent 42%),
			#f7fafc;
		color: #263238;
		text-align: center;
	}

	.not-found__content {
		width: min(100%, 620px);
	}

	.not-found__code {
		margin: 0;
		color: #25a9c8;
		font-size: clamp(96px, 22vw, 180px);
		font-weight: 800;
		line-height: 0.9;
		letter-spacing: 0;
	}

	.not-found__eyebrow {
		margin: 28px 0 10px;
		color: #0f766e;
		font-size: 16px;
		font-weight: 700;
		letter-spacing: 0;
	}

	h1 {
		margin: 0;
		font-size: clamp(24px, 4vw, 36px);
		line-height: 1.3;
		letter-spacing: 0;
	}

	.not-found__description {
		margin: 16px auto 0;
		max-width: 440px;
		color: #64748b;
		font-size: 15px;
		line-height: 1.8;
	}

	.not-found__actions {
		display: flex;
		justify-content: center;
		gap: 12px;
		margin-top: 28px;
	}

	.not-found__actions .button {
		margin: 0;
	}

	@media screen and (max-width: 480px) {
		.not-found {
			padding: 24px 16px;
		}

		.not-found__actions {
			flex-direction: column;
		}

		.not-found__actions .button {
			width: 100%;
		}
	}
</style>
