<script lang="jsx" setup>
import { onMounted, ref, reactive } from 'vue';
import { ElButton, ElMessageBox } from "element-plus";
import { Edit, Delete, Plus } from "@element-plus/icons-vue";
import { invoke } from '@tauri-apps/api/core';

onMounted(() => {
  dbconn_list_api();
})

const dbconn_list = ref([]);
const add_drawer = ref(false);
const diff_report_src = ref();
const diff_report_dst = ref();

const dbconn_form = reactive({
  driver: 'mysql',
  name: '127.0.0.1',
  host: '127.0.0.1',
  port: 3306,
  username: 'root',
  password: 'root',
  database: '',
})

const dbconn_list_api = async () => {
  const res = await invoke("dbconn_list");
  dbconn_list.value = res;
}

const edit_dbconn_api = async () => {
  await invoke("edit_dbconn", {
    conn: {
      ...dbconn_form
    }
  });
}

const del_dbconn_api = async (id) => {
  await invoke("del_dbconn", { id: id });
}

const cancelAddConnDrawer = () => {
  add_drawer.value = false
}

const onEditConnSubmit = () => {
  edit_dbconn_api();
  add_drawer.value = false
  dbconn_list_api();
}

const handleEdit = (row) => {
  add_drawer.value = true;

  dbconn_form.id = row.id;
  dbconn_form.driver = row.driver;
  dbconn_form.name = row.name;
  dbconn_form.host = row.host;
  dbconn_form.port = row.port;
  dbconn_form.username = row.username;
  dbconn_form.password = row.password;
  dbconn_form.database = row.database;
}

const handleDelete = (id) => {
  del_dbconn_api(id);
  dbconn_list_api();
}
</script>

<template>
  <el-main>
    <el-row :gutter="20">
      <el-col :span="16">
        <div class="grid-content ep-bg-purple">
          <el-table :data="dbconn_list" style="width: 100%" max-height="500">
            <el-table-column prop="id" label="ID" />
            <el-table-column prop="name" label="连接名" />
            <el-table-column prop="host" label="主机名" />
            <el-table-column prop="port" label="端口" />
            <el-table-column prop="username" label="用户名" />
            <el-table-column prop="database" label="数据库" />
            <el-table-column fixed="right" label="操作">
              <template #default="scope">
                <el-button size="small" :icon="Edit" @click="handleEdit(scope.row)" />
                <el-button size="small" type="danger" :icon="Delete" @click="handleDelete(scope.row.id)" />
              </template>
            </el-table-column>
          </el-table>
          <el-button class="mt-4" style="width: 100%" :icon="Plus" @click="add_drawer = true" />
        </div>
      </el-col>
      <el-col :span="8">
        <div class="grid-content ep-bg-purple">
          <el-form :inline="true" label-width="auto">
            <el-form-item label="差异报告">
              <label>fff</label>
              <el-select v-model="ddiff_report_src" placeholder="Select">
                <el-option v-for="conn in dbconn_list" :key="conn.id" :label="conn.name" :value="conn.id" />
              </el-select>
              <label>fff</label>
              <el-select v-model="ddiff_report_dst" placeholder="Select">
                <el-option v-for="conn in dbconn_list" :key="conn.id" :label="conn.name" :value="conn.id" />
              </el-select>
            </el-form-item>
          </el-form>

          <el-form :inline="true" label-width="auto">
            <el-form-item label="连接名">
              <el-input v-model="dbconn_form.name" />
            </el-form-item>
            <el-form-item label="主机名">
              <el-input v-model="dbconn_form.host" />
            </el-form-item>
            <el-form-item label="端口">
              <el-input-number v-model="dbconn_form.port" :min="0" :max="65535" />
            </el-form-item>
            <el-form-item label="用户名">
              <el-input v-model="dbconn_form.username" />
            </el-form-item>
            <el-form-item label="密码">
              <el-input v-model="dbconn_form.password" type="password" />
            </el-form-item>
            <el-form-item label="数据库">
              <el-input v-model="dbconn_form.database" />
            </el-form-item>
          </el-form>
        </div>
      </el-col>
    </el-row>

    <el-drawer v-model="add_drawer" direction="ttb" size="70%">
      <template #header>
        <h4>添加/编辑数据库连接</h4>
      </template>
      <template #default>
        <div>
          <el-form :model="dbconn_form" label-width="auto" style="max-width: 600px">
            <el-form-item label="方言">
              <el-select v-model="dbconn_form.driver">
                <el-option label="MySQL" value="mysql" />
                <el-option label="PgSQL" value="postgres" />
                <el-option label="Sqlite" value="sqlite" />
              </el-select>
            </el-form-item>
            <el-form-item label="连接名">
              <el-input v-model="dbconn_form.name" />
            </el-form-item>
            <el-form-item label="主机名">
              <el-input v-model="dbconn_form.host" />
            </el-form-item>
            <el-form-item label="端口">
              <el-input-number v-model="dbconn_form.port" :min="0" :max="65535" />
            </el-form-item>
            <el-form-item label="用户名">
              <el-input v-model="dbconn_form.username" />
            </el-form-item>
            <el-form-item label="密码">
              <el-input v-model="dbconn_form.password" type="password" />
            </el-form-item>
            <el-form-item label="数据库">
              <el-input v-model="dbconn_form.database" />
            </el-form-item>
          </el-form>
        </div>
      </template>
      <template #footer>
        <div style="flex: auto">
          <el-button @click="cancelAddConnDrawer">取消</el-button>
          <el-button type="primary" @click="onEditConnSubmit">保存</el-button>
        </div>
      </template>
    </el-drawer>
  </el-main>
</template>

<style scoped></style>
