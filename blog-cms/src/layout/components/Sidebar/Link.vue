<template>
  <component :is="type" v-bind="linkProps(to)">
    <slot />
  </component>
</template>

<script setup>
import {computed} from 'vue'
import {isExternal as isExternalUrl} from '@/util/validate'

const props = defineProps({to: {type: String, required: true}})
const external = computed(() => isExternalUrl(props.to))
const type = computed(() => external.value ? 'a' : 'router-link')
const linkProps = computed(() => external.value
  ? {href: props.to, target: '_blank', rel: 'noopener'}
  : {to: props.to})
</script>


