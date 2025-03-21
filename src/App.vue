<script setup>
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {listen} from '@tauri-apps/api/event';

const clipList = ref([]);

listen('clipboard_update', (event) => {
  if (clipList.value.length >= 50) {
    clipList.value.pop();
  }
  clipList.value.unshift(event.payload);
});

</script>

<template>
  <el-scrollbar style="height:550px">
    <template v-if="clipList.length > 0">
      <div v-for="(item, i) in clipList" :key="i" class="row">
        <div class="scrollbar-item" :style="{boxShadow:`var(--el-box-shadow-lighter)`}">{{ item }}</div>
      </div>
    </template>
    <template v-else>
      <el-empty description="No Data"/>
    </template>
  </el-scrollbar>
</template>

<style scoped>
.scrollbar-item {
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
  cursor: pointer;
}

.scrollbar-item:hover {
  box-shadow: var(--el-box-shadow-light) !important;
}
</style>
