(window["webpackJsonp"]=window["webpackJsonp"]||[]).push([["KindList"],{"4f45":function(t,e,n){"use strict";n.r(e);var a=function(){var t=this,e=t.$createElement,n=t._self._c||e;return n("div",{staticClass:"container"},[n("bulma-table",{attrs:{name:"kind",names:["Тип тренировки","Сокращенное наименование","Заметка"],columns:["name","short_name","note"],tableData:t.list,tableClasses:"is-narrow is-striped fullwidth",headClasses:["","","is-hidden-mobile"],cellTypes:["text","text","text"],pagination:"",hyper:"",search:"",adding:""}})],1)},s=[],i=n("a84d"),l=n("c546"),o={name:"KindList",components:{"bulma-table":i["a"]},mixins:[l["a"]],mounted(){this.fetchData("kind/list","KindList")},methods:{createList(t){let e=[];return t&&(e=t.map(t=>{const e=[t.name,t.short_name,t.note],n=t;return n.str=e.join(" ").toLowerCase(),n})),e}}},r=o,d=n("2877"),c=Object(d["a"])(r,a,s,!1,null,null,null);e["default"]=c.exports}}]);
//# sourceMappingURL=KindList.4724592a.js.map