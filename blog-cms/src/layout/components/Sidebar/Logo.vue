<template>
	<div class="sidebar-logo-container" :class="{'collapse':collapse}">
		<router-link class="sidebar-logo-link" to="/">
			<img v-if="logo" :src="logo" class="sidebar-logo">
			<h1 v-if="!collapse" class="sidebar-title">{{ title }}</h1>
			<h1 v-else-if="!logo" class="sidebar-title">{{ title }}</h1>
		</router-link>
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
	.sidebar-logo-container {
		position: relative;
		width: 100%;
		height: 58px;
		background: rgba(255, 255, 255, 0.92);
		border-bottom: 1px solid rgba(148, 163, 184, 0.18);
		overflow: hidden;

		& .sidebar-logo-link {
			height: 100%;
			width: 100%;
			display: flex;
			align-items: center;
			text-decoration: none;

			& .sidebar-logo {
				width: 32px;
				height: 32px;
				flex: 0 0 32px;
				margin: 0 12px 0 18px;
				border-radius: 8px;
			}

			& .sidebar-title {
				min-width: 0;
				margin: 0;
				color: #172033;
				font-weight: 750;
				font-size: 16px;
				font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
				overflow: hidden;
				text-overflow: ellipsis;
				white-space: nowrap;
			}
		}

		&.collapse {
			.sidebar-logo-link {
				justify-content: center;
			}

			.sidebar-logo {
				margin: 0;
			}
		}
	}
</style>


