(window["webpackJsonp"]=window["webpackJsonp"]||[]).push([["ScopeList"],{"1be4":function(e,t,s){"use strict";s.r(t);var a=function(){var e=this,t=e.$createElement,s=e._self._c||t;return s("div",{staticClass:"container"},[s("bulma-table",{attrs:{name:"scope",names:["Сфера деятельности","Заметка"],columns:["name","note"],tableData:e.list,tableClasses:"is-narrow is-striped fullwidth",headClasses:["","is-hidden-mobile"],cellTypes:["text","text"],pagination:"",hyper:"",search:"",adding:""}})],1)},n=[],i=s("a84d"),l=s("c546"),o={name:"ScopeList",components:{"bulma-table":i["a"]},mixins:[l["a"]],mounted(){this.fetchData("scope/list","ScopeList")},methods:{createList(e){let t=[];return e&&(t=e.map(e=>{const t=[e.name,e.note];return e.str=t.join(" ").toLowerCase(),e})),t}}},c=o,r=s("2877"),p=Object(r["a"])(c,a,n,!1,null,null,null);t["default"]=p.exports}}]);
//# sourceMappingURL=ScopeList.6a59011e.js.map