<template>
	<div>
		<el-tabs v-model="activeTab" type="card">
			<el-tab-pane label="基础设置" name="basic">
				<el-card>
					<el-form label-position="right" label-width="100px">
						<el-form-item :label="item.nameZh" v-for="item in basicSettings" :key="item.id">
							<el-input v-model="item.value" size="mini"></el-input>
						</el-form-item>
					</el-form>
				</el-card>
			</el-tab-pane>
			<el-tab-pane label="SEO设置" name="seo">
				<el-card>
					<el-form label-position="right" label-width="110px">
						<el-form-item label="网站描述">
							<el-input
								v-model="seoSettings.siteDescription.value"
								type="textarea"
								:rows="4"
								placeholder="用于首页和默认页面的 meta description，不填则前台输出空"
							></el-input>
						</el-form-item>
						<el-form-item label="网站关键词">
							<el-input
								v-model="seoSettings.siteKeywords.value"
								placeholder="多个关键词用英文逗号分隔，不填则前台输出空"
							></el-input>
						</el-form-item>
					</el-form>
				</el-card>
			</el-tab-pane>
			<el-tab-pane label="资料卡" name="profile">
				<el-card>
					<el-form label-position="right" label-width="100px">
						<el-form-item :label="item.nameZh" v-for="item in typeMap.type2" :key="item.id">
							<div v-if="item.nameEn=='favorite'">
								<el-col :span="20">
									<el-input v-model="item.value" size="mini"></el-input>
								</el-col>
								<el-col :span="4">
									<el-button type="danger" size="mini" icon="el-icon-delete" @click="deleteFavorite(item)">删除</el-button>
								</el-col>
							</div>
							<div v-else>
								<el-input v-model="item.value" size="mini"></el-input>
							</div>
						</el-form-item>
						<el-button type="primary" size="mini" icon="el-icon-plus" @click="addFavorite">添加自定义</el-button>
					</el-form>
				</el-card>
			</el-tab-pane>
			<el-tab-pane label="页脚徽标" name="footer">
				<el-card>
					<el-form :inline="true" v-for="badge in typeMap.type3" :key="badge.id">
						<el-form-item label="title">
							<el-input v-model="badge.value.title" size="mini"></el-input>
						</el-form-item>
						<el-form-item label="url">
							<el-input v-model="badge.value.url" size="mini"></el-input>
						</el-form-item>
						<el-form-item label="subject">
							<el-input v-model="badge.value.subject" size="mini"></el-input>
						</el-form-item>
						<el-form-item label="value">
							<el-input v-model="badge.value.value" size="mini"></el-input>
						</el-form-item>
						<el-form-item label="color">
							<el-input v-model="badge.value.color" size="mini"></el-input>
						</el-form-item>
						<el-form-item>
							<el-button type="danger" size="mini" icon="el-icon-delete" @click="deleteBadge(badge)">删除</el-button>
						</el-form-item>
					</el-form>
					<el-button type="primary" size="mini" icon="el-icon-plus" @click="addBadge">添加 badge</el-button>
				</el-card>
			</el-tab-pane>
		</el-tabs>

		<div style="text-align: right;margin-top: 30px">
			<el-button type="primary" icon="el-icon-check" @click="submit">保存</el-button>
		</div>
	</div>
</template>

<script>
	import Breadcrumb from "@/components/Breadcrumb";
	import {getSiteSettingData, update} from "@/api/siteSetting";
	import _ from 'lodash'

	export default {
		name: "SiteSetting",
		components: {Breadcrumb},
		data() {
			return {
				activeTab: "basic",
				deleteIds: [],
				typeMap: {},
				seoSettings: {
					siteDescription: this.createSetting('siteDescription', '网站描述'),
					siteKeywords: this.createSetting('siteKeywords', '网站关键词')
				}
			}
		},
		created() {
			this.getData()
		},
		computed: {
			basicSettings() {
				return (this.typeMap.type1 || []).filter(item => !['siteDescription', 'siteKeywords'].includes(item.nameEn))
			}
		},
		methods: {
			getData() {
				getSiteSettingData().then(res => {
					this.typeMap = res.data
					this.ensureSeoSettings()
					res.data.type3.forEach(item => {
						item.value = JSON.parse(item.value)
					})
				})
			},
			createSetting(nameEn, nameZh) {
				return {
					key: nameEn,
					nameEn,
					nameZh,
					type: 1,
					value: ''
				}
			},
			ensureSeoSettings() {
				this.seoSettings = {
					siteDescription: this.createSetting('siteDescription', '网站描述'),
					siteKeywords: this.createSetting('siteKeywords', '网站关键词')
				}
				const type1 = this.typeMap.type1 || []
				Object.keys(this.seoSettings).forEach(key => {
					const setting = type1.find(item => item.nameEn === key)
					if (setting) {
						this.seoSettings[key] = setting
					} else {
						type1.push(this.seoSettings[key])
					}
				})
				this.typeMap.type1 = type1
			},
			addFavorite() {
				this.typeMap.type2.push({
					key: Date.now(),
					nameEn: "favorite",
					nameZh: "自定义",
					type: 2,
					value: "{\"title\":\"\",\"content\":\"\"}"
				})
			},
			addBadge() {
				this.typeMap.type3.push({
					key: Date.now(),
					nameEn: "badge",
					nameZh: "徽标",
					type: 3,
					value: {
						color: "",
						subject: "",
						title: "",
						url: "",
						value: ""
					}
				})
			},
			deleteFavorite(favorite) {
				let arr = this.typeMap.type2
				if (favorite.id) {
					this.deleteIds.push(favorite.id)
					arr.forEach((item, index) => {
						if (item.id === favorite.id) {
							arr.splice(index, 1)
							return
						}
					})
				} else {
					arr.forEach((item, index) => {
						if (item.key === favorite.key) {
							arr.splice(index, 1)
							return
						}
					})
				}
			},
			deleteBadge(badge) {
				let arr = this.typeMap.type3
				if (badge.id) {
					this.deleteIds.push(badge.id)
					arr.forEach((item, index) => {
						if (item.id === badge.id) {
							arr.splice(index, 1)
							return
						}
					})
				} else {
					arr.forEach((item, index) => {
						if (item.key === badge.key) {
							arr.splice(index, 1)
							return
						}
					})
				}
			},
			submit() {
				const result = _.cloneDeep(this.typeMap)
				result.type3.forEach(item => {
					item.value = JSON.stringify(item.value)
				})
				let updateArr = []
				updateArr.push(...result.type1)
				updateArr.push(...result.type2)
				updateArr.push(...result.type3)
				update(updateArr, this.deleteIds).then(res => {
					this.deleteIds = []
					this.getData()
					this.msgSuccess(res.msg)
				})
			}
		}
	}
</script>

<style scoped>

</style>


