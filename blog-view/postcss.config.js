/**
 * PostCSS 配置：PurgeCSS 裁剪未使用的 CSS（重点：semantic-ui 257KB 整包）
 * - content：从 .vue/.js/index.html 提取用到的 class
 * - safelist：覆盖运行时/数据库动态拼接的 class（语义 UI 组件类、m-* 工具类、
 *   element-plus 组件类、viewerjs、标签颜色等），避免误删样式
 */
module.exports = {
	plugins: [
		require('@fullhuman/postcss-purgecss')({
			content: ['./index.html', './src/**/*.{vue,js}'],
			defaultExtractor: content => content.match(/[\w-/:]+(?<!:)/g) || [],
			safelist: {
				standard: [
					// 语义 UI 动态颜色（标签/分类颜色来自数据库）
					'red', 'orange', 'yellow', 'olive', 'green', 'teal', 'blue', 'violet',
					'purple', 'pink', 'brown', 'grey', 'gray', 'black', 'white',
					// 尺寸/状态（部分由 JS 或后端拼接）
					'mini', 'tiny', 'small', 'medium', 'large', 'big', 'huge', 'massive',
					'active', 'disabled', 'hidden', 'visible', 'loading', 'inverted', 'basic',
					'compact', 'circular', 'rounded', 'floated', 'right', 'left', 'center',
					'middle', 'top', 'bottom', 'attached', 'padded', 'segment', 'segments',
					'container', 'grid', 'row', 'column', 'header', 'content', 'description',
					'meta', 'extra', 'comment', 'comments', 'avatar', 'label', 'item', 'list',
					'menu', 'divider', 'image', 'message', 'icon', 'input', 'form', 'field',
					'button', 'card', 'modal', 'popup', 'tooltip', 'transition', 'fade',
					'dimmed', 'blurring', 'stacked', 'stackable', 'mobile', 'tablet', 'computer',
					'only', 'fluid', 'borderless', 'fitted', 'selection', 'dropdown', 'search',
					'pointing', 'corner', 'ribbon', 'floating', 'equal', 'wide', 'stretched',
					'relaxed', 'divided', 'celled', 'aligned', 'justified', 'centered',
					'vertical', 'horizontal', 'tabular', 'primary', 'secondary', 'positive',
					'negative', 'warning', 'error', 'info', 'success', 'open', 'ui',
					'item', 'anchor', 'text', 'date', 'metadata', 'left', 'right'
				],
				greedy: [
					// 语义 UI 组件根类（ui xxx）
					/^ui /,
					// 自定义 m-* 工具类（base.css 中的间距/文字工具）
					/^m-/,
					// element-plus 组件类与状态类
					/^el-/, /^is-/, /^js-/,
					// viewerjs 图片预览类（运行时添加）
					/^viewer-/,
					// element-plus 遮罩/popper 类
					/^v-modal/, /^v-popper/,
					// element-plus 过渡动画类
					/^zoom-in-/, /^fade-/, /^slide-/, /^collapse-/,
					// 自定义页面/组件类（Blog 阅读页、文章卡片等）
					/^reader-/, /^article-/, /^tag-/,
					// 通用布局/工具（避免误删）
					/^(mt|mb|ml|mr|pt|pb|pl|pr|px|py|mx|my)-/,
					/^(flex|grid|text|font|bg|border|rounded|shadow|opacity|cursor|overflow|position|items|justify|content|self|gap|space|whitespace|break|max|min|w|h|top|bottom|left|right)-/
				]
			}
		})
	]
}
