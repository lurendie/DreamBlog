import {onMounted} from 'vue'
import {storeToRefs} from 'pinia'
import {useAppStore} from '@/store'

export function useFixIosBug(subMenuRef) {
  const {device} = storeToRefs(useAppStore())
  onMounted(() => {
    const subMenu = subMenuRef.value
    if (!subMenu) return
    const handleMouseleave = subMenu.handleMouseleave
    subMenu.handleMouseleave = event => {
      if (device.value === 'mobile') return
      handleMouseleave(event)
    }
  })
}
