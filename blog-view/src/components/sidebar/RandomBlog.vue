<template>
	<!--随机文章-->
	<div class="ui segments m-box sidebar-panel">
		<div class="ui secondary segment sidebar-panel__title"><i class="bookmark icon"></i>随机文章</div>
		<div class="ui yellow segment sidebar-panel__body">
			<div class="ui divided items">
				<div class="m-item" v-for="blog in safeRandomBlogList" :key="blog.id" @click.prevent="toBlog(blog)">
					<div class="img" :style="{'background-image':'url(' + blog.firstPicture + ')'}"></div>
					<div class="info">
				<div class="date">{{ dateFormat(blog.createTime, 'YYYY-MM-DD') }}</div>
						<div class="title">{{ blog.title }}</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	export default {
		name: "RandomBlog",
		props: {
			randomBlogList: {
				type: Array,
				required: true
			},
		},
		computed: {
			safeRandomBlogList() {
				if (!Array.isArray(this.randomBlogList)) {
					return []
				}
				return this.randomBlogList.filter(blog => blog && typeof blog === 'object' && blog.title)
			}
		},
		methods: {
			toBlog(blog) {
				this.$store.dispatch('goBlogPage', blog)
			}
		}
	}
</script>

<style scoped>
	.secondary.segment {
		padding: 12px 14px !important;
	}

	.ui.divided.items .m-item:first-child {
		margin-top: 0;
	}

	.ui.divided.items .m-item {
		margin-top: 1rem;
		height: 8rem;
		position: relative;
		overflow: hidden;
		border-radius: 8px;
		cursor: pointer;
		user-select: none;
		box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.2);
	}

	.ui.divided.items .m-item .img {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		object-fit: cover;
		background-position-x: center;
		background-position-y: center;
		background-size: cover;
	}

	.ui.divided.items .m-item .info {
		z-index: 1;
		background: linear-gradient(to bottom, rgba(0, 0, 0, 0), rgba(0, 0, 0, 0.8));
		position: absolute;
		left: 0;
		right: 0;
		bottom: 0;
		padding: .5rem !important;
		font-size: 12px;
		color: white;
	}

	.sidebar-panel {
		overflow: hidden;
	}

	.sidebar-panel__body {
		padding: 12px !important;
	}

	.ui.divided.items .m-item .info .title {
		overflow: hidden;
		text-overflow: ellipsis;
		display: -webkit-box;
		-webkit-box-orient: vertical;
		-webkit-line-clamp: 1;
		word-break: break-word;
	}
</style>
