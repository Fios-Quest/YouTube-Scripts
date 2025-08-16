// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><li class="part-title">IRISS</li><li class="chapter-item expanded "><a href="iriss/000-iriss.html"><strong aria-hidden="true">1.</strong> Welcome to IRISS</a></li><li class="chapter-item expanded "><a href="iriss/001-iriss.html"><strong aria-hidden="true">2.</strong> Setting Up</a></li><li class="chapter-item expanded "><a href="iriss/002-iriss.html"><strong aria-hidden="true">3.</strong> Memory</a></li><li class="chapter-item expanded "><a href="iriss/003-iriss.html"><strong aria-hidden="true">4.</strong> Data Types</a></li><li class="chapter-item expanded "><a href="iriss/004-iriss.html"><strong aria-hidden="true">5.</strong> Control Flow</a></li><li class="chapter-item expanded "><a href="iriss/005-iriss.html"><strong aria-hidden="true">6.</strong> Functions</a></li><li class="chapter-item expanded "><a href="iriss/006-iriss.html"><strong aria-hidden="true">7.</strong> Tests</a></li><li class="chapter-item expanded "><a href="iriss/007-iriss.html"><strong aria-hidden="true">8.</strong> Documentation</a></li><li class="chapter-item expanded "><a href="iriss/008-iriss.html"><strong aria-hidden="true">9.</strong> Clippy and Friends</a></li><li class="chapter-item expanded "><a href="iriss/009-iriss.html"><strong aria-hidden="true">10.</strong> Implementing Types</a></li><li class="chapter-item expanded "><a href="iriss/010-iriss.html"><strong aria-hidden="true">11.</strong> Traits</a></li><li class="chapter-item expanded "><a href="iriss/011-common-traits-part-1.html"><strong aria-hidden="true">12.</strong> Common Traits - Part 1</a></li><li class="chapter-item expanded "><a href="iriss/012-common-traits-part-2.html"><strong aria-hidden="true">13.</strong> Common Traits - Part 2</a></li><li class="chapter-item expanded "><a href="iriss/013-collections.html"><strong aria-hidden="true">14.</strong> Collections</a></li><li class="chapter-item expanded "><a href="iriss/014-iterators.html"><strong aria-hidden="true">15.</strong> Iterators</a></li><li class="chapter-item expanded "><a href="iriss/015-threads.html"><strong aria-hidden="true">16.</strong> Threads</a></li><li class="chapter-item expanded "><a href="iriss/016-macros.html"><strong aria-hidden="true">17.</strong> Macros</a></li><li class="chapter-item expanded affix "><li class="part-title">Channel Labs</li><li class="chapter-item expanded "><a href="channel-news/001-fios-quest.html"><strong aria-hidden="true">18.</strong> Long Live Fio&#39;s Quest</a></li><li class="chapter-item expanded affix "><li class="part-title">Shorts</li><li class="chapter-item expanded "><a href="shorts/001-sixty-seconds.html"><strong aria-hidden="true">19.</strong> Lifetimes in 60s</a></li><li class="chapter-item expanded affix "><li class="part-title">Streams</li><li class="chapter-item expanded "><a href="streams/lets-build/job-tracker-05.html"><strong aria-hidden="true">20.</strong> Job Tracker 05</a></li><li class="chapter-item expanded "><a href="streams/lets-build/job-tracker-06.html"><strong aria-hidden="true">21.</strong> Job Tracker 06</a></li><li class="chapter-item expanded "><a href="streams/lets-build/job-tracker-07.html"><strong aria-hidden="true">22.</strong> Job Tracker 07</a></li><li class="chapter-item expanded "><a href="streams/lets-build/job-tracker-10.html"><strong aria-hidden="true">23.</strong> Job Tracker 10</a></li><li class="chapter-item expanded "><a href="streams/lets-build/job-tracker-11.html"><strong aria-hidden="true">24.</strong> Job Tracker 11</a></li><li class="chapter-item expanded "><a href="streams/lets-build/job-tracker-12.html"><strong aria-hidden="true">25.</strong> Job Tracker 12</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
