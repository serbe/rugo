(window["webpackJsonp"]=window["webpackJsonp"]||[]).push([["CertificateList"],{f30b:function(t,e,a){"use strict";a.r(e);var n=function(){var t=this,e=t.$createElement,a=t._self._c||e;return a("div",{staticClass:"container"},[a("bulma-table",{attrs:{name:"certificate",names:["Номер","ФИО","УМЦ","Дата","Заметка"],columns:["num","contact_name","company_name","cert_date","note"],tableData:t.list,tableClasses:"is-narrow is-striped fullwidth",headClasses:["","","is-hidden-mobile","nowrap","is-hidden-mobile"],cellTypes:["text","text","text","text","text"],pagination:"",hyper:"",search:"",adding:""}})],1)},i=[],s=a("a84d"),c=a("c546"),r={name:"CertificateList",components:{"bulma-table":s["a"]},mixins:[c["a"]],mounted(){this.fetchData("certificate/list","CertificateList")},methods:{createList(t){let e=[];return t&&(e=t.map(t=>{const e=[t.num,t.contact_name,t.company_name,t.cert_date,t.note],a=t;return a.str=e.join(" ").toLowerCase(),a})),e}}},l=r,o=a("2877"),m=Object(o["a"])(l,n,i,!1,null,null,null);e["default"]=m.exports}}]);
//# sourceMappingURL=CertificateList.040c8417.js.map