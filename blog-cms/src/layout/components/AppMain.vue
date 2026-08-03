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
		/*50 = navbar  */
		min-height: calc(100vh - 50px);
		width: 100%;
		position: relative;
		overflow: hidden;
		padding: 20px;
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


