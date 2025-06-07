use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="page-content max-w-4xl mx-auto">
            <div class="text-center mb-12">
                <h2 class="text-4xl font-bold text-gray-900 mb-4">"About Quote Client"</h2>
                <p class="text-lg text-gray-600 leading-relaxed">
                    "A modern web client for browsing and managing inspirational quotes. "
                    "Built with Rust and WebAssembly for blazing-fast performance."
                </p>
            </div>

            <div class="grid md:grid-cols-2 gap-8 mb-12">
                <div class="bg-white rounded-lg shadow-lg p-6">
                    <h3 class="text-2xl font-semibold text-gray-900 mb-4 flex items-center">
                        <span class="text-blue-600 mr-2">"🔧"</span>
                        "Technology Stack"
                    </h3>
                    <ul class="space-y-3">
                        <li class="flex items-center">
                            <span class="w-2 h-2 bg-orange-500 rounded-full mr-3"></span>
                            <span class="font-medium">"Rust"</span>
                            <span class="text-gray-600 ml-2">"- Systems programming language"</span>
                        </li>
                        <li class="flex items-center">
                            <span class="w-2 h-2 bg-purple-500 rounded-full mr-3"></span>
                            <span class="font-medium">"Leptos"</span>
                            <span class="text-gray-600 ml-2">"- Reactive web framework"</span>
                        </li>
                        <li class="flex items-center">
                            <span class="w-2 h-2 bg-green-500 rounded-full mr-3"></span>
                            <span class="font-medium">"WebAssembly"</span>
                            <span class="text-gray-600 ml-2">"- High-performance runtime"</span>
                        </li>
                        <li class="flex items-center">
                            <span class="w-2 h-2 bg-blue-500 rounded-full mr-3"></span>
                            <span class="font-medium">"Trunk"</span>
                            <span class="text-gray-600 ml-2">"- Build tool and dev server"</span>
                        </li>
                        <li class="flex items-center">
                            <span class="w-2 h-2 bg-cyan-500 rounded-full mr-3"></span>
                            <span class="font-medium">"Tailwind CSS"</span>
                            <span class="text-gray-600 ml-2">"- Utility-first styling"</span>
                        </li>
                    </ul>
                </div>

                <div class="bg-white rounded-lg shadow-lg p-6">
                    <h3 class="text-2xl font-semibold text-gray-900 mb-4 flex items-center">
                        <span class="text-green-600 mr-2">"✨"</span>
                        "Features"
                    </h3>
                    <ul class="space-y-3">
                        <li class="flex items-start">
                            <span class="text-green-500 mr-3 mt-1">"•"</span>
                            <span>"Browse all quotes with pagination"</span>
                        </li>
                        <li class="flex items-start">
                            <span class="text-green-500 mr-3 mt-1">"•"</span>
                            <span>"Discover random inspirational quotes"</span>
                        </li>
                        <li class="flex items-start">
                            <span class="text-green-500 mr-3 mt-1">"•"</span>
                            <span>"Create and submit new quotes"</span>
                        </li>
                        <li class="flex items-start">
                            <span class="text-green-500 mr-3 mt-1">"•"</span>
                            <span>"Update and edit existing quotes"</span>
                        </li>
                        <li class="flex items-start">
                            <span class="text-green-500 mr-3 mt-1">"•"</span>
                            <span>"Delete quotes with confirmation"</span>
                        </li>
                        <li class="flex items-start">
                            <span class="text-green-500 mr-3 mt-1">"•"</span>
                            <span>"Responsive design for all devices"</span>
                        </li>
                        <li class="flex items-start">
                            <span class="text-green-500 mr-3 mt-1">"•"</span>
                            <span>"Real-time updates and state management"</span>
                        </li>
                    </ul>
                </div>
            </div>

            <div class="text-center">
                <div class="bg-gradient-to-r from-blue-500 to-purple-600 rounded-lg p-6 text-white">
                    <h3 class="text-xl font-semibold mb-2">"Built with Modern Web Standards"</h3>
                    <p class="text-blue-100">
                        "Experience the power of Rust in the browser with client-side rendering "
                        "and a REST API backend for seamless quote management."
                    </p>
                </div>
            </div>
        </div>
    }
}