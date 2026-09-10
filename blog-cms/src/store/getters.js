import {storeToRefs} from 'pinia'
import {useAppStore} from './modules/app'

export function useStoreGetters() {
  const app = useAppStore()
  const {sidebar, device} = storeToRefs(app)
  return {sidebar, device}
}
