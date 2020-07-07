#include <cstring>
#include <iostream>

#include "rust_web_api_caller.h"

using namespace std;

ostream &operator<<(ostream &os, const Error &e)
{
    switch (e)
    {
    case Error::InvalidResponse:
        os << "Invalid response";
        break;
    case Error::InvalidUrl:
        os << "Invalid url";
        break;
    case Error::NetworkError:
        os << "Network error";
        break;
    }
    return os;
}

int main()
{
    const char *url = "https://google.com";

    Result<Graph, Error> result = get_graph_from_url(reinterpret_cast<const uint8_t *>(url), strlen(url));

    if (result.tag == Result<Graph, Error>::Tag::Ok)
    {
        cout << "OK!" << endl;
        Graph graph = result.ok._0;
        cout << "id: " << graph.id << endl;
        cout << "name: " << string(reinterpret_cast<const char *>(graph.name.ptr), graph.name.len) << endl;
        cout << "X label: " << string(reinterpret_cast<const char *>(graph.x_label.ptr), graph.x_label.len) << endl;
        cout << "Y label: " << string(reinterpret_cast<const char *>(graph.y_label.ptr), graph.y_label.len) << endl;
    }
    else
    {
        cout << "ERR " << result.err._0 << endl;
    }
}