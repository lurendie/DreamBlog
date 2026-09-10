<template>
	<div class="navbar">
		<div class="navbar-left">
			<hamburger :is-active="sidebar.opened" class="hamburger-container" @toggleClick="toggleSideBar"/>

			<breadcrumb class="breadcrumb-container"/>
		</div>

		<div class="right-menu">
			<el-button text @click="$router.push('/blog/write')">写文章</el-button>
			<el-dropdown class="avatar-container" trigger="click">
				<div class="avatar-wrapper">
					<img :src="user.avatar" class="user-avatar" v-if="user">
				</div>
				<template #dropdown><el-dropdown-menu class="user-dropdown">
					<a target="_blank" href="https://github.com/Naccl/NBlog">
						<el-dropdown-item>
							<SvgIcon icon-class="github" class-name="svg"/>
							<span>GitHub</span>
						</el-dropdown-item>
					</a>
					<el-dropdown-item @click="logout">
						<SvgIcon icon-class="logout" class-name="svg"/>
						<span>退出</span>
					</el-dropdown-item>
				</el-dropdown-menu></template>
			</el-dropdown>
		</div>
	</div>
</template>

<script setup>
	import {ref, onMounted} from 'vue'
	import {useRouter} from 'vue-router'
	import {storeToRefs} from 'pinia'
	import {ElMessage} from 'element-plus'
	import Breadcrumb from '@/components/Breadcrumb'
	import Hamburger from '@/components/Hamburger'
	import SvgIcon from '@/components/SvgIcon'
	import {useAppStore} from '@/store'
	import {clearLoginState, getStoredUser} from '@/util/storage'
	import {logout as logoutApi} from '@/api/login'

	const router = useRouter()
	const appStore = useAppStore()
	const {sidebar} = storeToRefs(appStore)
	const user = ref(null)
	const toggleSideBar = () => appStore.toggleSideBar()
	const getUserInfo = () => {
		user.value = getStoredUser()
		if (!user.value) { clearLoginState(); router.push('/login') }
	}
	const logout = () => logoutApi().catch(() => {}).finally(() => {
		clearLoginState(); router.push('/login'); ElMessage.success('退出成功')
	})
	onMounted(getUserInfo)
</script>

<style lang="scss" scoped>
	.navbar {
		display: flex;
		height: 58px;
		align-items: center;
		justify-content: space-between;
		overflow: visible;
		position: relative;
		background: rgba(255, 255, 255, 0.88);
		border-bottom: 1px solid rgba(148, 163, 184, 0.18);
		box-shadow: 0 10px 28px rgba(15, 23, 42, .04);
		backdrop-filter: blur(12px);
		user-select: none;

		.navbar-left {
			display: flex;
			min-width: 0;
			align-items: center;
			height: 100%;
		}

		.hamburger-container {
			line-height: 54px;
			height: 100%;
			float: left;
			cursor: pointer;
			transition: background .3s;
			-webkit-tap-highlight-color: transparent;

				&:hover {
					background: rgba(15, 118, 110, .06)
				}
			}

		.breadcrumb-container {
			float: left;
		}

		.right-menu {
			display: flex;
			align-items: center;
			gap: 12px;
			float: none;
			height: 100%;
			padding-right: 20px;
			line-height: 58px;

			&:focus {
				outline: none;
			}

			.right-menu-item {
				display: inline-block;
				padding: 0 8px;
				height: 100%;
				font-size: 18px;
				color: #5a5e66;
				vertical-align: text-bottom;

				&.hover-effect {
					cursor: pointer;
					transition: background .3s;

					&:hover {
						background: rgba(0, 0, 0, .025)
					}
				}
			}

			.avatar-container {
				margin-right: 0;

				.avatar-wrapper {
					display: flex;
					align-items: center;
					margin-top: 0;
					position: relative;

					.user-avatar {
						cursor: pointer;
						width: 40px;
						height: 40px;
						border: 2px solid rgba(148, 163, 184, 0.18);
						border-radius: 50%;
						object-fit: cover;
					}

					.el-icon-caret-bottom {
						cursor: pointer;
						position: absolute;
						right: -20px;
						top: 0px;
						font-size: 12px;
					}
				}
			}
		}
	}

	.user-dropdown .svg {
		margin-right: 5px;
	}

	.el-dropdown-menu {
		margin: 7px 0 0 0 !important;
		padding: 0 !important;
		border: 0 !important;
	}
</style>


