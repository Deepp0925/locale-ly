!(function (e, t) {
  "object" == typeof exports && "undefined" != typeof module
    ? (module.exports = t())
    : "function" == typeof define && define.amd
    ? define(t)
    : ((e || self).autosize = t());
})(this, function () {
  var e = new Map();
  function t(t) {
    var o = e.get(t);
    o && o.destroy();
  }
  function o(t) {
    var o = e.get(t);
    o && o.update();
  }
  var r = null;
  return (
    "undefined" == typeof window
      ? (((r = function (e) {
          return e;
        }).destroy = function (e) {
          return e;
        }),
        (r.update = function (e) {
          return e;
        }))
      : (((r = function (t, o) {
          return (
            t &&
              Array.prototype.forEach.call(t.length ? t : [t], function (t) {
                return (function (t) {
                  if (
                    t &&
                    t.nodeName &&
                    "TEXTAREA" === t.nodeName &&
                    !e.has(t)
                  ) {
                    var o,
                      r = null,
                      n = window.getComputedStyle(t),
                      i =
                        ((o = t.value),
                        function () {
                          s({
                            testForHeightReduction:
                              "" === o || !t.value.startsWith(o),
                            restoreTextAlign: null,
                          }),
                            (o = t.value);
                        }),
                      l = function (o) {
                        t.removeEventListener("autosize:destroy", l),
                          t.removeEventListener("autosize:update", a),
                          t.removeEventListener("input", i),
                          window.removeEventListener("resize", a),
                          Object.keys(o).forEach(function (e) {
                            return (t.style[e] = o[e]);
                          }),
                          e.delete(t);
                      }.bind(t, {
                        height: t.style.height,
                        resize: t.style.resize,
                        textAlign: t.style.textAlign,
                        overflowY: t.style.overflowY,
                        overflowX: t.style.overflowX,
                        wordWrap: t.style.wordWrap,
                      });
                    t.addEventListener("autosize:destroy", l),
                      t.addEventListener("autosize:update", a),
                      t.addEventListener("input", i),
                      window.addEventListener("resize", a),
                      (t.style.overflowX = "hidden"),
                      (t.style.wordWrap = "break-word"),
                      e.set(t, { destroy: l, update: a }),
                      a();
                  }
                  function s(e) {
                    var o,
                      i,
                      l = e.restoreTextAlign,
                      a = void 0 === l ? null : l,
                      d = e.testForHeightReduction,
                      u = void 0 === d || d,
                      f = n.overflowY;
                    if (
                      0 !== t.scrollHeight &&
                      ("vertical" === n.resize
                        ? (t.style.resize = "none")
                        : "both" === n.resize &&
                          (t.style.resize = "horizontal"),
                      u &&
                        ((o = (function (e) {
                          for (
                            var t = [];
                            e &&
                            e.parentNode &&
                            e.parentNode instanceof Element;

                          )
                            e.parentNode.scrollTop &&
                              t.push([e.parentNode, e.parentNode.scrollTop]),
                              (e = e.parentNode);
                          return function () {
                            return t.forEach(function (e) {
                              var t = e[0],
                                o = e[1];
                              (t.style.scrollBehavior = "auto"),
                                (t.scrollTop = o),
                                (t.style.scrollBehavior = null);
                            });
                          };
                        })(t)),
                        (t.style.height = "")),
                      (i =
                        "content-box" === n.boxSizing
                          ? t.scrollHeight -
                            (parseFloat(n.paddingTop) +
                              parseFloat(n.paddingBottom))
                          : t.scrollHeight +
                            parseFloat(n.borderTopWidth) +
                            parseFloat(n.borderBottomWidth)),
                      "none" !== n.maxHeight && i > parseFloat(n.maxHeight)
                        ? ("hidden" === n.overflowY &&
                            (t.style.overflow = "scroll"),
                          (i = parseFloat(n.maxHeight)))
                        : "hidden" !== n.overflowY &&
                          (t.style.overflow = "hidden"),
                      (t.style.height = i + "px"),
                      a && (t.style.textAlign = a),
                      o && o(),
                      r !== i &&
                        (t.dispatchEvent(
                          new Event("autosize:resized", { bubbles: !0 })
                        ),
                        (r = i)),
                      f !== n.overflow && !a)
                    ) {
                      var c = n.textAlign;
                      "hidden" === n.overflow &&
                        (t.style.textAlign = "start" === c ? "end" : "start"),
                        s({ restoreTextAlign: c, testForHeightReduction: !0 });
                    }
                  }
                  function a() {
                    s({ testForHeightReduction: !0, restoreTextAlign: null });
                  }
                })(t);
              }),
            t
          );
        }).destroy = function (e) {
          return e && Array.prototype.forEach.call(e.length ? e : [e], t), e;
        }),
        (r.update = function (e) {
          return e && Array.prototype.forEach.call(e.length ? e : [e], o), e;
        })),
    r
  );
});

let textareas = document.querySelectorAll("textarea");

for (let textarea of textareas) {
  textarea.addEventListener("input", autosize);
  textarea.addEventListener("focus", autosize);
}

autosize(document.querySelectorAll("textarea"));
console.log(autosize);