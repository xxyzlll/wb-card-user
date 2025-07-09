import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";

// 引入Element Plus
import ElementPlus from "element-plus";
import "element-plus/dist/index.css";

// 引入中文语言包
import zhCn from "element-plus/dist/locale/zh-cn.mjs";

// 引入Element Plus图标
import * as ElementPlusIconsVue from '@element-plus/icons-vue';

const app = createApp(App);

// 注册所有图标组件
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component);
}

app.use(router)
  .use(ElementPlus, {
    locale: zhCn,
  })
  .mount("#app");
