(window["webpackJsonp"]=window["webpackJsonp"]||[]).push([["RankList"],{dff9:function(t,e,a){"use strict";a.r(e);var n=function(){var t=this,e=t.$createElement,a=t._self._c||e;return a("div",{staticClass:"container"},[a("bulma-table",{attrs:{name:"rank",names:["Наименование чина","Заметка"],columns:["name","note"],tableData:t.list,tableClasses:"is-narrow is-striped fullwidth",headClasses:["","is-hidden-mobile"],cellTypes:["text","text"],pagination:"",hyper:"",search:"",adding:""}})],1)},s=[],i=a("a84d"),l=a("c546"),r={name:"RankList",components:{"bulma-table":i["a"]},mixins:[l["a"]],mounted(){this.fetchData("rank/list","RankList")},methods:{createList(t){let e=[];return t&&(e=t.map(t=>{const e=[t.name,t.note];return t.str=e.join(" ").toLowerCase(),t})),e}}},o=r,c=a("2877"),u=Object(c["a"])(o,n,s,!1,null,null,null);e["default"]=u.exports}}]);
//# sourceMappingURL=RankList.d778551f.js.map