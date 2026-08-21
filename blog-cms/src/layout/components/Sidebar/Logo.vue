<template>
	<div class="sidebar-logo-container" :class="{'collapse':collapse}">
		<transition name="sidebarLogoFade">
			<router-link v-if="collapse" key="collapse" class="sidebar-logo-link" to="/">
				<img v-if="logo" :src="logo" class="sidebar-logo">
				<h1 v-else class="sidebar-title">{{ title }} </h1>
			</router-link>
			<router-link v-else key="expand" class="sidebar-logo-link" to="/">
				<img v-if="logo" :src="logo" class="sidebar-logo">
				<h1 class="sidebar-title">{{ title }} </h1>
			</router-link>
		</transition>
	</div>
</template>

<script>
	export default {
		name: 'SidebarLogo',
		props: {
			collapse: {
				type: Boolean,
				required: true
			}
		},
		data() {
			return {
				title: this.$store.state.settings.title,
				logo: this.$store.state.settings.logo
			}
		}
	}
</script>

<style lang="scss" scoped>
	.sidebarLogoFade-enter-active {
		transition: opacity 1.5s;
	}

	.sidebarLogoFade-enter,
	.sidebarLogoFade-leave-to {
		opacity: 0;
	}

	.sidebar-logo-container {
		position: relative;
		width: 100%;
		height: 58px;
		line-height: 58px;
		background: rgba(255, 255, 255, 0.92);
		border-bottom: 1px solid rgba(148, 163, 184, 0.18);
		text-align: left;
		overflow: hidden;

		& .sidebar-logo-link {
			height: 100%;
			width: 100%;

			& .sidebar-logo {
				width: 32px;
				height: 32px;
				vertical-align: middle;
				margin: 0 12px 0 18px;
				border-radius: 8px;
			}

			& .sidebar-title {
				display: inline-block;
				margin: 0;
				color: #172033;
				font-weight: 750;
				line-height: 58px;
				font-size: 16px;
				font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
				vertical-align: middle;
			}
		}

		&.collapse {
			.sidebar-logo {
				margin-right: 0px;
				margin-left: 11px;
			}
		}
	}
</style>


