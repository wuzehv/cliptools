<script setup>
import {ref, onMounted} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {listen} from '@tauri-apps/api/event';
import {register, unregisterAll} from '@tauri-apps/plugin-global-shortcut';
import {getCurrentWindow} from "@tauri-apps/api/window";
import {DocumentCopy} from "@element-plus/icons-vue";
import {Menu} from '@tauri-apps/api/menu';
import {checkPrefix, transform} from "./assets/callback.js";
import {show} from '@tauri-apps/api/app';

getCurrentWindow().onCloseRequested((event) => {
  getCurrentWindow().hide();
  event.preventDefault();
});

getCurrentWindow().onFocusChanged(({ payload: focused }) => {
  if (!focused) {
    getCurrentWindow().hide();
  }
});

const clipList = ref([]);

listen('clipboard_update', (event) => {
  if (clipList.value.length >= 50) {
    clipList.value.pop();
  }
  clipList.value.unshift(event.payload);
});

onMounted(async () => {
  const k = 'Super+C';
  await unregisterAll();
  await register(k, () => {
    getCurrentWindow().show();
    getCurrentWindow().setFocus();
  });
});

async function sendText(text) {
  await getCurrentWindow().hide();
  await invoke("set_window_text", {text: text});
}

</script>

<template>
  <el-scrollbar style="height:550px">
    <template v-if="clipList.length > 0">
      <div v-for="(item, i) in clipList" :key="i" class="row">
        <div class="scrollbar-item" @click="sendText(item)" :style="{boxShadow:`var(--el-box-shadow-lighter)`}">{{
            item
          }}
          <el-icon v-if="checkPrefix(item)" class="icon" :size="20"
                   @click.stop="sendText(transform(item))">
            <DocumentCopy/>
          </el-icon>
        </div>
      </div>
    </template>
    <template v-else>
      <el-empty description="No Data"/>
    </template>
  </el-scrollbar>
</template>

<style scoped>
.scrollbar-item {
  position: relative;
  padding: 5px;
  display: flex;
  height: 90px;
  margin: 10px;
  text-align: left;
  border-radius: 4px;
  color: #505050;
  overflow: hidden;
  font-size: 13px;
  box-sizing: border-box;
  white-space: break-spaces;
  word-break: break-word;
}

.scrollbar-item:hover {
  box-shadow: 0px 0px 12px rgba(0, 0, 0, 0.28) !important;
}

.icon {
  position: absolute;
  bottom: 10px;
  right: 10px;
  cursor: pointer;
  color: #1596ff;
  font-size: 20px;
  transition: color 0.3s;
}
</style>
