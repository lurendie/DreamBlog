<template>
	<div>
		<el-alert title="图床配置及用法请查看：https://github.com/Naccl/PictureHosting" type="warning" show-icon v-if="hintShow"></el-alert>
		<el-card>
			<template #header>
				<span>GitHub配置</span>
			</template>
			<el-row>
				<el-col>
					<el-input placeholder="请输入token进行初始化" v-model="githubToken" :clearable="true" show-password @keyup.enter="searchGithubUser" style="min-width: 500px">
						<template #append>
							<el-button icon="el-icon-search" :disabled="!githubToken" @click="searchGithubUser">查询</el-button>
						</template>
					</el-input>
				</el-col>
			</el-row>
			<el-row>
				<el-col>
					<span class="middle">当前用户：</span>
					<el-avatar :size="50" :src="githubUserInfo.avatar_url">User</el-avatar>
					<span class="middle">{{ githubUserInfo.login }}</span>
				</el-col>
			</el-row>
			<el-row>
				<el-col>
					<el-button type="primary" size="medium" icon="el-icon-check" :disabled="!githubToken" @click="saveGithub(true)">保存配置</el-button>
					<el-button type="info" size="medium" icon="el-icon-close" @click="saveGithub(false)">清除配置</el-button>
				</el-col>
			</el-row>
		</el-card>

		<el-card>
			<template #header>
				<span>又拍云存储配置</span>
			</template>
			<el-form :model="upyunConfig" label-width="100px">
				<el-form-item label="操作员名称">
					<el-input v-model="upyunConfig.username"></el-input>
				</el-form-item>
				<el-form-item label="操作员密码">
					<el-input v-model="upyunConfig.password" show-password></el-input>
				</el-form-item>
				<el-form-item label="存储空间名">
					<el-input v-model="upyunConfig.bucketName"></el-input>
				</el-form-item>
				<el-form-item label="CDN访问域名">
					<el-input v-model="upyunConfig.domain"></el-input>
				</el-form-item>
				<el-button type="primary" size="medium" icon="el-icon-check" :disabled="!isUpyunSave" @click="saveUpyun(true)">保存配置</el-button>
				<el-button type="info" size="medium" icon="el-icon-close" @click="saveUpyun(false)">清除配置</el-button>
			</el-form>
		</el-card>

		<el-card>
			<template #header>
				<span>腾讯云存储配置</span>
			</template>
			<el-form :model="txyunConfig" label-width="100px">
				<el-form-item label="secret-id">
					<el-input v-model="txyunConfig.secretId" show-password></el-input>
				</el-form-item>
				<el-form-item label="secret-key">
					<el-input v-model="txyunConfig.secretKey" show-password></el-input>
				</el-form-item>
				<el-form-item label="存储空间名">
					<el-input v-model="txyunConfig.bucketName"></el-input>
				</el-form-item>
				<el-form-item label="地域">
					<el-input v-model="txyunConfig.region"></el-input>
				</el-form-item>
				<el-form-item label="CDN访问域名">
					<el-input v-model="txyunConfig.domain"></el-input>
				</el-form-item>
				<el-button type="primary" size="medium" icon="el-icon-check" :disabled="!isTxyunSave" @click="saveTxyun(true)">保存配置</el-button>
				<el-button type="info" size="medium" icon="el-icon-close" @click="saveTxyun(false)">清除配置</el-button>
			</el-form>
		</el-card>
	</div>
</template>

<script>
import {
	deleteConfig,
	getConfigs,
	getGithubUser,
	saveGithubConfig,
	saveTxyunConfig,
	saveUpyunConfig
} from "@/api/pictureHosting";
import {getStoredUser} from "@/util/storage";

export default {
	name: "Setting",
	data() {
		return {
			githubToken: '',
			githubUserInfo: {
				login: '未配置'
			},
			isGithubSave: false,
			hintShow: false,
			upyunConfig: {
				username: '',
				password: '',
				bucketName: '',
				domain: ''
			},
			txyunConfig: {
				secretId: '',
				secretKey: '',
				bucketName: '',
				region: '',
				domain: ''
			},
		}
	},
	computed: {
		isUpyunSave() {
			return this.upyunConfig.username && this.upyunConfig.password && this.upyunConfig.bucketName && this.upyunConfig.domain
		},
		isTxyunSave() {
			return this.txyunConfig.secretId && this.txyunConfig.secretKey && this.txyunConfig.bucketName && this.txyunConfig.region && this.txyunConfig.domain
		}
	},
	created() {
		this.loadConfigs()

		const user = getStoredUser()
		if (user && user.role !== 'ROLE_admin') {
			//对于访客模式，增加个提示
			this.hintShow = true
		}
	}
	,
	methods: {
		loadConfigs() {
			getConfigs().then(res => {
				const configs = res.data || {}
				if (configs.github && configs.github.configured && configs.github.userInfo) {
					this.githubUserInfo = configs.github.userInfo
					this.isGithubSave = true
				} else {
					this.githubUserInfo = {login: '未配置'}
					this.isGithubSave = false
				}
				if (configs.upyun && configs.upyun.configured) {
					this.upyunConfig.bucketName = configs.upyun.bucketName || ''
					this.upyunConfig.domain = configs.upyun.domain || ''
				}
				if (configs.txyun && configs.txyun.configured) {
					this.txyunConfig.bucketName = configs.txyun.bucketName || ''
					this.txyunConfig.region = configs.txyun.region || ''
					this.txyunConfig.domain = configs.txyun.domain || ''
				}
			})
		},
		// 获取用户信息
		searchGithubUser() {
			getGithubUser(this.githubToken).then(res => {
				this.githubUserInfo = res.data
				this.isGithubSave = true
			})
		}
		,
		saveGithub(save) {
			if (save) {
				saveGithubConfig(this.githubToken).then(res => {
					this.githubUserInfo = res.data
					this.isGithubSave = true
					this.githubToken = ''
					this.msgSuccess('保存成功')
				})
			} else {
				deleteConfig('github').then(() => {
					this.githubToken = ''
					this.githubUserInfo = {login: '未配置'}
					this.isGithubSave = false
					this.msgSuccess('清除成功')
				})
			}
		}
		,
		saveUpyun(save) {
			if (save) {
				saveUpyunConfig(this.upyunConfig).then(() => {
					this.upyunConfig.username = ''
					this.upyunConfig.password = ''
					this.msgSuccess('保存成功')
				})
			} else {
				deleteConfig('upyun').then(() => {
					this.upyunConfig = {username: '', password: '', bucketName: '', domain: ''}
					this.msgSuccess('清除成功')
				})
			}
		}
		,
		saveTxyun(save) {
			if (save) {
				saveTxyunConfig(this.txyunConfig).then(() => {
					this.txyunConfig.secretId = ''
					this.txyunConfig.secretKey = ''
					this.msgSuccess('保存成功')
				})
			} else {
				deleteConfig('txyun').then(() => {
					this.txyunConfig = {secretId: '', secretKey: '', bucketName: '', region: '', domain: ''}
					this.msgSuccess('清除成功')
				})
			}
		}
	}
	,
}
</script>

<style scoped>
.el-alert + .el-row, .el-row + .el-row {
	margin-top: 20px;
}

.el-avatar {
	vertical-align: middle;
	margin-right: 15px;
}

.middle {
	vertical-align: middle;
}

.el-card {
	width: 50%;
}

.el-card + .el-card {
	margin-top: 20px;
}
</style>


