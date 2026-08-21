<template>
	<div>
		<div class="ui top attached segment" style="text-align: center">
			<h2 class="m-text-500">我的动态</h2>
		</div>
		<div class="ui attached segment m-padding-bottom-large">
			<div class="moments">
				<div class="moment" v-for="(moment,index) in momentList" :key="index">
					<div class="avatar">
						<img :src="$store.state.introduction.avatar">
					</div>
					<div class="ui card">
						<div class="content m-top">
							<span style="font-weight: 700">{{ $store.state.introduction.name }}</span>
							<span class="right floated">{{ dateFromNow(moment.createTime) }}</span>
						</div>
						<div class="content typo" :class="{'privacy':!moment.published}" v-lazy-container="{selector: 'img'}" v-viewer v-safe-html="moment.content"></div>
						<div class="extra content">
							<a class="left floated" @click="like(moment.id)">
								<i class="heart icon" :class="isLike(moment.id)?'like-color':'outline'"></i>{{ moment.likes }}
							</a>
						</div>
					</div>
				</div>
			</div>

			<LightPagination
				class="pagination"
				:current-page="pageNum"
				:page-count="totalPage"
				@current-change="handleCurrentChange"
			/>
		</div>
	</div>
</template>

<script>
	import {getMomentListByPageNum, likeMoment} from "@/api/moment";
	import { updateSeo } from '@/util/seo'
	import {safeParseArray} from '@/util/storage'
	import { directive as viewerDirective } from 'v-viewer'
	import 'viewerjs/dist/viewer.css'
	import LightPagination from "@/components/common/LightPagination.vue";

	export default {
		name: "Moments",
		components: {LightPagination},
		directives: {
			viewer: viewerDirective()
		},
		data() {
			return {
				//用localStorage本地存储已点赞的动态id数组（safeParseArray 防御损坏/非数组内容导致页面崩溃）
				likeMomentIds: safeParseArray(window.localStorage.getItem('likeMomentIds')),
				momentList: [],
				pageNum: 1,
				totalPage: 0
			}
		},
		created() {
			this.getMomentList()
		},
		watch: {
			likeMomentIds(newValue) {
				//将likeMomentIds最新值的json数据保存到localStorage
				const likedIds = Array.isArray(newValue) ? newValue : []
				if (likedIds !== newValue) {
					this.likeMomentIds = likedIds
					return
				}
				window.localStorage.setItem('likeMomentIds', JSON.stringify(likedIds))
			}
		},
		methods: {
			isLike(id) {
				return Array.isArray(this.likeMomentIds) && this.likeMomentIds.indexOf(id) > -1
			},
			getMomentList() {
				//博主身份由 httpOnly Cookie 自动携带
				getMomentListByPageNum('', this.pageNum).then(res => {
					if (res.code === 200) {
						const data = res.data || {}
						this.momentList = Array.isArray(data.list) ? data.list : []
						this.totalPage = Number(data.totalPage || data.pages || 0)
						updateSeo({
							title: '动态',
							description: this.$store.state.siteInfo?.siteDescription || '',
							keywords: this.$store.state.siteInfo?.siteKeywords || '',
							path: this.$route.fullPath,
						})
					} else {
						this.msgError(res.msg)
					}
				}).catch(() => {
					this.msgError("请求失败")
				})
			},
			handleCurrentChange(newPage) {
				this.scrollToTop()
				this.pageNum = newPage
				this.getMomentList()
			},
			like(id) {
				if (this.isLike(id)) {
					this.$notify({
						title: '不可以重复点赞哦',
						type: 'warning'
					})
					return
				}
				likeMoment(id).then(res => {
					if (res.code === 200) {
						this.$notify({
							title: res.msg,
							type: 'success'
						})
						if (!Array.isArray(this.likeMomentIds)) {
							this.likeMomentIds = []
						}
						this.likeMomentIds.push(id)
						this.momentList.forEach(item => {
							if (item.id === id) {
								return item.likes++
							}
						})
					} else {
						this.$notify({
							title: res.msg,
							type: 'warning'
						})
					}
				}).catch(() => {
					this.$notify({
						title: '异常错误',
						type: 'error'
					})
				})
			}
		}
	}
</script>

<style scoped>
	.avatar {
		margin-left: -62.5px;
		float: left !important;
	}

	.avatar img {
		height: 45px;
		width: 45px;
		border-radius: 500px;
	}

	.moments {
		margin-left: 26px !important;
		padding-left: 40px !important;
		border-left: 1px solid #dee5e7 !important;
	}

	.moment {
		margin-top: 30px;
	}

	.moment:first-child {
		margin-top: 0 !important;
	}

	.card {
		width: 100% !important;
	}

	.card:before {
		border-width: 0 0 1px 1px !important;
		transform: translateX(-50%) translateY(-50%) rotate(45deg) !important;
		bottom: auto !important;
		right: auto !important;
		top: 22px !important;
		left: 0 !important;
		position: absolute !important;
		content: '' !important;
		background-image: none !important;
		z-index: 2 !important;
		width: 12px !important;
		height: 12px !important;
		transition: background .1s ease !important;
		background-color: inherit !important;
		border-style: solid !important;
		border-color: #d4d4d5 !important;
	}

	.content.m-top {
		padding: 10px 14px !important;
	}

	.content .right.floated {
		font-size: 12px !important;
	}

	.content.typo * {
		font-size: 14px !important;
	}

	.extra.content {
		padding: 5px 14px !important;
	}

	.extra.content a {
		color: rgba(0, 0, 0, 0.7) !important;
		font-size: 12px !important;
	}

	.extra.content a:hover {
		color: red !important;
	}

	.extra.content .like-color {
		color: red !important;
	}

	.extra.content i {
		font-size: 12px !important;
	}

	.pagination {
		text-align: center;
		margin-top: 3em;
	}

	.privacy {
		background: repeating-linear-gradient(145deg, #f2f2f2, #f2f2f2 15px, #fff 0, #fff 30px) !important;
	}

	@media screen and (max-width: 767px) {
		.ui.attached.segment.m-padding-bottom-large {
			padding-left: 0.85rem !important;
			padding-right: 0.85rem !important;
		}

		.moments {
			margin-left: 0 !important;
			padding-left: 16px !important;
		}

		.avatar {
			margin-left: -38px;
		}

		.avatar img {
			width: 34px;
			height: 34px;
		}

		.moment {
			margin-top: 20px;
		}

		.pagination {
			margin-top: 1.6rem;
		}
	}
</style>
