(window["webpackJsonp"]=window["webpackJsonp"]||[]).push([["EducationList"],{"482e":function(t,e,a){"use strict";a.r(e);var n=function(){var t=this,e=t.$createElement,a=t._self._c||e;return a("div",{staticClass:"container"},[a("bulma-table",{attrs:{name:"education",names:["Обучаемый","Должность ГО ЧС","Начало обучения","Конец обучения"],columns:["contact_name","post_name","start_str","end_str"],tableData:t.list,tableClasses:"is-narrow is-striped fullwidth",headClasses:["","is-hidden-mobile","","is-hidden-mobile"],cellTypes:["text","text","text","text"],pagination:"",hyper:"",search:"",adding:""}})],1)},s=[],i=a("a84d"),o=a("c546"),r={name:"EducationList",components:{"bulma-table":i["a"]},mixins:[o["a"]],mounted:function(){this.fetchData("education/list","EducationList")},methods:{createList:function(t){var e=[];return t&&(e=t.map(function(t){var e=[t.contact_name,t.start_str,t.end_str];return t.str=e.join(" ").toLowerCase(),t})),e}}},c=r,l=a("2877"),u=Object(l["a"])(c,n,s,!1,null,null,null);e["default"]=u.exports}}]);
//# sourceMappingURL=EducationList-legacy.3beab5da.js.map