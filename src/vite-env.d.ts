/// <reference types="vite/client" />

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}

//全局window对象类型信息
interface Window {
  router: Router,
  fs: any
}

type UnlistenFn = () => void;

interface AppInfo {
  name: string, //应用名，将显示在应用菜单面板上
  id: string, //应用id，如果component属性为不为null，那么该选项必填，否则可选
  icon: string,   //图标所在路径
  feature: Array<string>, //该应用可处理的特点，当用户输出指定特点的内容时将被显示在推荐页，目前支持：filename、url、dir_path、file_path,explorer
  only_feature: boolean, //是否只在触发特点时才显示
  weight: number, //应用权重，随用户使用次数进行计算
  component: Component | string | null, //带界面的内置应用需填入组件，外部的插件应用需要填入index.html文件位置，如果都不是，则应填入null代表无界面应用
  setup: () => void, //当用户点击该app时将被立即执行的函数
}

interface SysAppInfo {
  name: string,
  icon: string,
  start: string,
}

interface FileInfo {
  name: string,
  path: string,
  time: number,
  ftype: number, //1：文件、2：目录
}

interface LocalApp {
  name: string;
  path: string; //路径作为id
  icon: string;
}

interface WebUrlApp {
  name: string,//网站名
  url: string,//网页路径，同时作为id
  icon: string,//网页图标
  on: boolean,//是否启动
  features: Array<string>, //可被匹配的特性
}

interface FileInfo {
  name: string,
  ext: string,
  path: string,
  time: number,
  isdir: boolean,
}
