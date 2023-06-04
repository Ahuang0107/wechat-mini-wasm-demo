// index.js
import init, {onLoad } from "wechat_mini_wasm_demo";
// 获取应用实例
const app = getApp()

Page({
  async onLoad() {
    await init("/pages/index/wechat_mini_wasm_demo_bg.wasm");
    onLoad();
  }
})
