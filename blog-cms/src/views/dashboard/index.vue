<template>
	<div class="dashboard-page">
		<PageHeader
			eyebrow="Dream Blog Console"
			title="Dashboard"
			:description="`${greeting}，${userName}。先看今天的访问和内容状态。`"
		>
			<template #actions>
				<el-button @click="$router.push('/blog/list')">文章管理</el-button>
				<el-button type="primary" @click="$router.push('/blog/write')">写文章</el-button>
			</template>
		</PageHeader>

		<div class="metric-grid">
			<MetricCard
				v-for="item in metricCards"
				:key="item.title"
				:title="item.title"
				:value="item.value"
				:description="item.description"
				:icon="item.icon"
				:accent="item.accent"
			/>
		</div>

		<div class="dashboard-grid">
			<ChartPanel title="访客地图" description="按城市聚合的访问分布" :height="380">
				<div ref="mapEcharts" class="chart-box"></div>
			</ChartPanel>
			<ChartPanel title="分类构成" description="分类下文章数量" :height="380">
				<div ref="categoryEcharts" class="chart-box"></div>
			</ChartPanel>
			<ChartPanel title="标签构成" description="标签下文章数量" :height="380">
				<div ref="tagEcharts" class="chart-box"></div>
			</ChartPanel>
			<ChartPanel title="访问趋势" description="最近一周 PV / UV" :height="420" class="dashboard-grid__wide">
				<div ref="visitRecordEcharts" class="chart-box"></div>
			</ChartPanel>
		</div>
	</div>
</template>

<script>
	import * as echarts from 'echarts/core'
	import {EffectScatterChart, LineChart, MapChart, PieChart, ScatterChart} from 'echarts/charts'
	import {GeoComponent, GridComponent, LegendComponent, TitleComponent, TooltipComponent} from 'echarts/components'
	import {CanvasRenderer} from 'echarts/renderers'
	import {getDashboard} from "@/api/dashboard";
	import ChartPanel from '@/components/ChartPanel'
	import MetricCard from '@/components/MetricCard'
	import PageHeader from '@/components/PageHeader'
	import {getStoredUser} from '@/util/storage'

	echarts.use([
		CanvasRenderer,
		EffectScatterChart,
		GeoComponent,
		GridComponent,
		LegendComponent,
		LineChart,
		MapChart,
		PieChart,
		ScatterChart,
		TitleComponent,
		TooltipComponent,
	])
	//echarts 5 不再内置中国地图，用项目内自带数据注册（省级粒度）
	import chinaJson from '@/util/china.json'
	echarts.registerMap('china', chinaJson)
	//城市经纬度数据来自 https://github.com/Naccl/region2coord
	import geoCoordMap from '@/util/city2coord.json'

	export default {
		name: "Dashboard",
		components: {ChartPanel, MetricCard, PageHeader},
		data() {
			return {
				user: getStoredUser(),
				pv: 0,
				uv: 0,
				blogCount: 0,
				commentCount: 0,
				categoryEcharts: null,
				tagEcharts: null,
				mapEcharts: null,
				visitRecordEcharts: null,
				_resizeHandler: null,
				categoryOption: {
					title: { show: false },
					color: ['#0f766e', '#475569', '#f59e0b', '#ef4444', '#14b8a6', '#64748b'],
					tooltip: {
						trigger: 'item',
						formatter: '{a} <br/>{b} : {c} ({d}%)'
					},
					legend: {
						left: 'center',
						top: 'bottom',
						data: []
					},
					series: [
						{
							name: '文章数量',
							type: 'pie',
							radius: [30, 110],
							roseType: 'area',
							data: []
						}
					]
				},
				tagOption: {
					title: { show: false },
					color: ['#0f766e', '#475569', '#f59e0b', '#ef4444', '#14b8a6', '#64748b'],
					tooltip: {
						trigger: 'item',
						formatter: '{a} <br/>{b} : {c} ({d}%)'
					},
					legend: {
						left: 'center',
						top: 'bottom',
						data: []
					},
					series: [
						{
							name: '文章数量',
							top: '-10%',
							type: 'pie',
							radius: [30, 110],
							roseType: 'area',
							data: []
						}
					]
				},
				//地图效果 reference https://www.jianshu.com/p/028525cbd080
				//reference https://echarts.apache.org/examples/zh/editor.html?c=map-polygon
				mapOption: {
					title: { show: false },
					tooltip: {
						show: false
					},
					geo: {
						map: "china",
						roam: false,//关闭拖拽
						zoom: 1.24,
						center: [104.2, 36],//调整地图位置
						//echarts 5 已移除 normal 层级，默认样式写在顶层
						label: {
							show: false,//关闭省份名展示
							fontSize: "10",
							color: "rgba(0,0,0,0.7)",
							emphasis: {
								show: false
							}
						},
						itemStyle: {
							areaColor: "#f8fafc",
							borderColor: "#d8e0ed",
							borderWidth: 1,//设置外层边框
							shadowBlur: 10,
							shadowOffsetY: 8,
							shadowOffsetX: 0,
							shadowColor: "rgba(30, 41, 59, 0.08)",
							emphasis: {
								areaColor: "#ecfdf5",
								shadowOffsetX: 0,
								shadowOffsetY: 0,
								shadowBlur: 5,
								borderWidth: 0,
								shadowColor: "rgba(0, 0, 0, 0.5)"
							}
						}
					},
					series: [
						{
							type: "map",
							map: "china",
							roam: false,
							zoom: 1.24,
							center: [104.2, 36],
							showLegendSymbol: false,
						label: {
							show: false,
							emphasis: {
								show: false
							}
						},
						itemStyle: {
							areaColor: "#f8fafc",
							borderColor: "#d8e0ed",
							borderWidth: 0.5,
							emphasis: {
								areaColor: "#ecfdf5",
								shadowOffsetX: 0,
								shadowOffsetY: 0,
								shadowBlur: 5,
								borderWidth: 0,
								shadowColor: "rgba(0, 0, 0, 0.5)"
							}
						}
						},
						{
							name: "",
							type: "scatter",
							coordinateSystem: "geo",
							data: [],
							symbol: "circle",
							symbolSize: 5,
							hoverSymbolSize: 10,
							tooltip: {
								formatter(value) {
									return value.data.name + "<br/>" + "访客数：" + value.data.uv
								},
								show: true
							},
							encode: {
								value: 2
							},
							label: {
								formatter: "{b}",
								position: "right",
								show: false
							},
							itemStyle: {
								color: "#0f766e"
							},
							emphasis: {
								label: {
									show: false
								}
							}
						},
						{
							name: "Top 5",
							type: "effectScatter",
							coordinateSystem: "geo",
							data: [],
							symbol: "circle",
							symbolSize: 12,
							tooltip: {
								formatter(value) {
									return value.data.name + "<br/>" + "访客数：" + value.data.uv
								},
								show: true
							},
							encode: {
								value: 2
							},
							showEffectOn: "render",
							rippleEffect: {
								brushType: "stroke",
								color: "#0f766e",
								period: 9,
								scale: 5
							},
							hoverAnimation: true,
							label: {
								formatter: "{b}",
								position: "right",
								show: true
							},
							itemStyle: {
								color: "#0f766e",
								shadowBlur: 2,
								shadowColor: "#333"
							},
							zlevel: 1
						}
					]
				},
				visitRecordOption: {
					xAxis: {
						data: [],
						boundaryGap: false,
						axisTick: {
							show: false
						}
					},
					grid: {
						left: 10,
						right: 20,
						top: 30,
						bottom: 0,
						containLabel: true
					},
					tooltip: {
						trigger: 'axis',
						axisPointer: {
							type: 'cross'
						},
						padding: [5, 10]
					},
					yAxis: {
						axisTick: {
							show: false
						}
					},
					legend: {
						data: ['访问量(PV)', '独立访客(UV)']
					},
					series: [
						{
							name: '访问量(PV)',
							smooth: true,
							type: 'line',
							itemStyle: {
								normal: {
									color: '#0f766e',
									lineStyle: {
										color: '#0f766e',
										width: 2
									}
								}
							},
							data: [],
							animationDuration: 2800,
							animationEasing: 'cubicInOut'
						},
						{
							name: '独立访客(UV)',
							smooth: true,
							type: 'line',
							itemStyle: {
								normal: {
									color: '#64748b',
									lineStyle: {
										color: '#64748b',
										width: 2
									},
									areaStyle: {
										color: 'rgba(100, 116, 139, 0.12)'
									}
								}
							},
							data: [],
							animationDuration: 2800,
							animationEasing: 'quadraticOut'
						}
					]
				},
			}
		},
		computed: {
			userName() {
				return this.user?.nickname || this.user?.username || '管理员'
			},
			greeting() {
				const hour = new Date().getHours()
				if (hour < 6) return '夜深了'
				if (hour < 9) return '早上好'
				if (hour < 12) return '上午好'
				if (hour < 14) return '中午好'
				if (hour < 18) return '下午好'
				return '晚上好'
			},
			metricCards() {
				return [
					{title: '今日 PV', value: this.pv, description: '页面访问量', icon: 'pv', accent: 'blue'},
					{title: '今日 UV', value: this.uv, description: '独立访客数', icon: 'yonghu', accent: 'indigo'},
					{title: '文章数', value: this.blogCount, description: '已收录文章', icon: 'article', accent: 'amber'},
					{title: '评论数', value: this.commentCount, description: '读者互动', icon: 'pinglun-blue', accent: 'violet'}
				]
			}
		},
		beforeUnmount() {
			//销毁 echarts 实例，避免路由切换后内存泄漏
			if (this._resizeHandler) {
				window.removeEventListener('resize', this._resizeHandler)
			}
			if (this.categoryEcharts) this.categoryEcharts.dispose()
			if (this.tagEcharts) this.tagEcharts.dispose()
			if (this.mapEcharts) this.mapEcharts.dispose()
			if (this.visitRecordEcharts) this.visitRecordEcharts.dispose()
		},
		mounted() {
			this.getData()
			this._resizeHandler = () => this.resizeCharts()
			window.addEventListener('resize', this._resizeHandler)
		},
		methods: {
			getData() {
				getDashboard().then(res => {
					this.pv = res.data.pv
					this.uv = res.data.uv
					this.blogCount = res.data.blogCount
					this.commentCount = res.data.commentCount
					//渲染分类数据
					this.categoryOption.legend.data = res.data.category.legend
					this.categoryOption.series[0].data = res.data.category.series
					this.initCategoryEcharts()
					//渲染标签数据
					this.tagOption.legend.data = res.data.tag.legend
					this.tagOption.series[0].data = res.data.tag.series
					this.initTagEcharts()
					//渲染访客地图数据
					let mapData = this.convertData(res.data.cityVisitor)
					this.mapOption.series[1].data = mapData
					//先拷贝再截取，避免 splice 破坏 series[1].data 对原数组的引用
					this.mapOption.series[2].data = [...mapData].splice(0, 5)
					this.initMapEcharts()
					//渲染一周访问量数据
					this.visitRecordOption.xAxis.data = res.data.visitRecord.date
					this.visitRecordOption.series[0].data = res.data.visitRecord.pv
					this.visitRecordOption.series[1].data = res.data.visitRecord.uv
					this.initVisitRecordEcharts()
				})
			},
			initCategoryEcharts() {
				this.categoryEcharts = echarts.init(this.$refs.categoryEcharts, 'light')
				this.categoryEcharts.setOption(this.categoryOption)
			},
			initTagEcharts() {
				this.tagEcharts = echarts.init(this.$refs.tagEcharts, 'light')
				this.tagEcharts.setOption(this.tagOption)
			},
			initMapEcharts() {
				this.mapEcharts = echarts.init(this.$refs.mapEcharts)
				this.mapEcharts.setOption(this.mapOption)
			},
			convertData(data) {
				let res = []
				for (let i = 0; i < data.length; i++) {
					let geoCoord = geoCoordMap[data[i].city]
					if (geoCoord) {
						res.push({
							name: data[i].city,
							value: geoCoord,
							uv: data[i].uv
						})
					}
				}
				return res
			},
			initVisitRecordEcharts() {
				this.visitRecordEcharts = echarts.init(this.$refs.visitRecordEcharts)
				this.visitRecordEcharts.setOption(this.visitRecordOption)
			},
			resizeCharts() {
				if (this.categoryEcharts) this.categoryEcharts.resize()
				if (this.tagEcharts) this.tagEcharts.resize()
				if (this.mapEcharts) this.mapEcharts.resize()
				if (this.visitRecordEcharts) this.visitRecordEcharts.resize()
			},
		}
	}
</script>

<style scoped>
	.dashboard-page {
		max-width: 1480px;
		margin: 0 auto;
	}

	.metric-grid {
		display: grid;
		grid-template-columns: repeat(4, minmax(0, 1fr));
		gap: 16px;
		margin-bottom: 18px;
	}

	.dashboard-grid {
		display: grid;
		grid-template-columns: repeat(3, minmax(0, 1fr));
		gap: 18px;
		align-items: stretch;
	}

	.dashboard-grid__wide {
		grid-column: span 3;
	}

	.chart-box {
		width: 100%;
		height: 100%;
		min-height: 280px;
	}

	@media screen and (max-width: 1200px) {
		.metric-grid {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}

		.dashboard-grid {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}

		.dashboard-grid__wide {
			grid-column: span 2;
		}
	}

	@media screen and (max-width: 768px) {
		.metric-grid,
		.dashboard-grid {
			display: block;
		}

		.metric-grid :deep(.metric-card),
		.dashboard-grid :deep(.chart-panel) {
			margin-bottom: 14px;
		}

		.dashboard-grid__wide {
			grid-column: auto;
		}
	}
</style>
