(window["webpackJsonp"]=window["webpackJsonp"]||[]).push([["PostList"],{dae2:function(t,e,a){"use strict";a.r(e);var s=function(){var t=this,e=t.$createElement,a=t._self._c||e;return a("div",{staticClass:"container"},[a("bulma-table",{attrs:{name:"post",names:["Наименование должности","ГО","Заметка"],columns:["name","go","note"],tableData:t.list,tableClasses:"is-narrow is-striped fullwidth",headClasses:["","w9","is-hidden-mobile"],cellTypes:["text","checkbox","text"],pagination:"",hyper:"",search:"",adding:""}})],1)},n=[],i=(a("7f7f"),a("a84d")),o=a("c546"),l={name:"PostList",components:{"bulma-table":i["a"]},mixins:[o["a"]],mounted:function(){this.fetchData("post/list","PostList")},methods:{createList:function(t){var e=[];return t&&(e=t.map(function(t){var e=[t.name,t.note];return t.str=e.join(" ").toLowerCase(),t})),e}}},r=l,c=a("2877"),u=Object(c["a"])(r,s,n,!1,null,null,null);e["default"]=u.exports}}]);
//# sourceMappingURL=PostList-legacy.5881f7a2.js.map