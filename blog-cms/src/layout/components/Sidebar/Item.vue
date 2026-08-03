<template>
  <component :is="icon" v-if="icon && icon.includes('el-icon')" class="sub-el-icon" />
  <svg-icon v-else-if="icon" :icon-class="icon" />
  <span v-if="title">{{ title }}</span>
</template>

<script>
export default {
  name: 'MenuItem',
  props: {
    icon: {
      type: String,
      default: ''
    },
    title: {
      type: String,
      default: ''
    }
  }
}
</script>

<style scoped>
.sub-el-icon {
  color: currentColor;
  width: 1em !important;
  height: 1em;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}
</style>


