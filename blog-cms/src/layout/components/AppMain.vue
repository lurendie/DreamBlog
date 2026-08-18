<template>
	<section class="app-main">
		<router-view v-slot="{ Component, route }">
			<transition name="fade-transform" mode="out-in">
				<component :is="Component" :key="route.path"/>
			</transition>
		</router-view>
	</section>
</template>

<script>
	export default {
		name: 'AppMain'
	}
</script>

<style scoped>
	.fixed-header + .app-main {
		padding-top: 60px;
	}

	.app-main {
		min-height: calc(100vh - 58px);
		width: 100%;
		position: relative;
		overflow-x: hidden;
		overflow-y: auto;
		padding: 26px;
		background:
			linear-gradient(180deg, rgba(37, 99, 235, 0.05), rgba(37, 99, 235, 0) 220px),
			#f6f8fb;
	}

	@media screen and (max-width: 768px) {
		.app-main {
			padding: 16px;
		}
	}
</style>

<style lang="scss">
	// fix css style bug in open el-dialog
	.el-popup-parent--hidden {
		.fixed-header {
			padding-right: 15px;
		}
	}
</style>


